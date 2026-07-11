//! Сборка приложения: роуты системных эндпоинтов и всех бизнес-доменов
//! + общий middleware.

use std::time::Duration;

use axum::Router;
use axum::http::StatusCode;
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
    Router::new()
        .merge(system::router())
        .merge(domains::router())
        // GraphQL: POST /graphql (запросы) + GET /graphql (песочница GraphiQL).
        .merge(graphql::router())
        // Swagger UI по /api + OpenAPI JSON по /api-docs/openapi.json.
        .merge(SwaggerUi::new("/api").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
