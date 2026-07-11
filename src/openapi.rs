//! Сборка OpenAPI-документа (Swagger).
//!
//! Схема генерируется из аннотаций `#[utoipa::path]` на хендлерах и
//! `#[derive(ToSchema)]` на DTO. Каждый новый эндпоинт добавляй в `paths(...)`,
//! а его модели — в `components(schemas(...))`.
//!
//! JSON доступен по `/api-docs/openapi.json`, интерактивный UI — по `/api`.

use utoipa::OpenApi;

/// OpenAPI-документ приложения.
#[derive(OpenApi)]
#[openapi(
    paths(crate::system::health::health),
    components(schemas(crate::system::health::HealthResponse)),
    tags((name = "system", description = "Системные эндпоинты"))
)]
pub struct ApiDoc;
