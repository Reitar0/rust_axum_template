//! Срез `health`: проверка живости сервиса и подключения к БД.

use axum::Router;
use axum::routing::get;

use crate::shared::state::AppState;

mod handler;

/// Роуты среза health.
pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(handler::health))
}
