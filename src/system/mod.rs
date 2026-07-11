//! Системные (инфраструктурные) эндпоинты: health-check и т.п.
//! Это не бизнес-логика — держим отдельно от `domains/`.

use axum::Router;
use axum::routing::get;

use crate::shared::state::AppState;

mod health;

/// Роуты системных эндпоинтов.
pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health::health))
}
