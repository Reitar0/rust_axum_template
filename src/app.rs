//! Сборка приложения: роуты системных эндпоинтов и всех бизнес-доменов
//! + общий middleware.

use std::time::Duration;

use axum::Router;
use axum::http::{HeaderValue, Method, StatusCode, header};
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::openapi::ApiDoc;
use crate::shared::state::AppState;
use crate::{domains, graphql, system};

/// Строит корневой `Router`: системные эндпоинты + роуты всех доменов,
/// сверху — сквозной middleware (логирование, таймаут).
pub fn build_router(state: AppState) -> Router {
    let cors = build_cors(&state.config.cors_allowed_origins);
    Router::new()
        .merge(system::router())
        .merge(domains::router())
        // GraphQL: POST /graphql (запросы) + GET /graphql (песочница GraphiQL).
        .merge(graphql::router())
        // Swagger UI по /api + OpenAPI JSON по /api-docs/openapi.json.
        .merge(SwaggerUi::new("/api").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // CORS: разрешённые origins берутся из конфига (env CORS_ALLOWED_ORIGINS).
        .layer(cors)
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Строит CORS-слой из списка разрешённых origins.
///
/// Разрешаются только явно указанные origins (никакого `*`), что безопасно и для
/// прода: в проде в `CORS_ALLOWED_ORIGINS` кладут реальный домен фронта.
fn build_cors(allowed_origins: &[String]) -> CorsLayer {
    let origins: Vec<HeaderValue> = allowed_origins
        .iter()
        .filter_map(|o| o.parse::<HeaderValue>().ok())
        .collect();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
        .allow_credentials(true)
}
