//! Сборка всего приложения: роуты всех срезов + общий middleware.

use std::time::Duration;

use axum::Router;
use axum::http::StatusCode;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::features;
use crate::shared::state::AppState;

/// Строит корневой `Router`, объединяя роуты всех вертикальных срезов
/// и навешивая сквозной middleware (логирование запросов, таймаут).
pub fn build_router(state: AppState) -> Router {
    Router::new()
        // Срезы — каждый добавляется через .merge(...).
        .merge(features::health::router())
        // Сквозной middleware.
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(TraceLayer::new_for_http())
        // Прокидываем общее состояние во все хендлеры.
        .with_state(state)
}
