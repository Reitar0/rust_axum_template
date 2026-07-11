//! Обработчик `GET /health`.

use axum::Json;
use axum::extract::State;
use serde::Serialize;
use utoipa::ToSchema;

use crate::shared::error::AppError;
use crate::shared::state::AppState;

/// Тело ответа health-check.
#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    /// Общий статус сервиса.
    status: &'static str,
    /// Статус подключения к базе данных.
    database: &'static str,
}

/// Проверка живости сервиса и подключения к БД (`SELECT 1`).
#[utoipa::path(
    get,
    path = "/health",
    responses((status = 200, description = "Сервис жив, база отвечает", body = HealthResponse)),
    tag = "system"
)]
pub async fn health(State(state): State<AppState>) -> Result<Json<HealthResponse>, AppError> {
    sqlx::query("SELECT 1").execute(&state.db).await?;

    Ok(Json(HealthResponse {
        status: "ok",
        database: "up",
    }))
}
