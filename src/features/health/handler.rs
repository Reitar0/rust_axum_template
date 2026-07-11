//! Обработчик `GET /health`.

use axum::Json;
use axum::extract::State;
use serde::Serialize;

use crate::shared::error::AppError;
use crate::shared::state::AppState;

/// Тело ответа health-check.
#[derive(Serialize)]
pub struct HealthResponse {
    /// Общий статус сервиса.
    status: &'static str,
    /// Статус подключения к базе данных.
    database: &'static str,
}

/// Проверяет, что сервис жив и база отвечает.
///
/// Выполняет тривиальный запрос `SELECT 1`. Если база недоступна, ошибка через
/// `?` превращается в `AppError::Database` → HTTP 500.
pub async fn health(State(state): State<AppState>) -> Result<Json<HealthResponse>, AppError> {
    sqlx::query("SELECT 1").execute(&state.db).await?;

    Ok(Json(HealthResponse {
        status: "ok",
        database: "up",
    }))
}
