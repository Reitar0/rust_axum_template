//! Единый тип ошибки приложения и его превращение в HTTP-ответ.
//!
//! Хендлеры возвращают `Result<T, AppError>`. Благодаря `impl IntoResponse`
//! axum сам преобразует ошибку в корректный HTTP-ответ, а оператор `?`
//! бесшовно пробрасывает ошибки нижних слоёв (sqlx, anyhow).

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// Ошибка приложения. Каждый вариант отображается в конкретный HTTP-статус.
///
/// Часть вариантов (`NotFound`, `Validation`, `Unauthorized`) — задел под
/// будущие срезы; пока они не конструируются, поэтому глушим `dead_code`.
#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// Сущность не найдена → 404.
    #[error("не найдено: {0}")]
    NotFound(String),

    /// Некорректные входные данные → 422.
    #[error("некорректные данные: {0}")]
    Validation(String),

    /// Не авторизован → 401. Задел под будущую аутентификацию.
    #[error("не авторизован")]
    Unauthorized,

    /// Ошибка базы данных → 500. Деталь пишется в лог, наружу — общее сообщение.
    #[error("ошибка базы данных")]
    Database(#[from] sqlx::Error),

    /// Прочие внутренние ошибки → 500.
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "не авторизован".to_string()),
            // Внутренние ошибки: подробность — в лог, клиенту — обезличенное сообщение,
            // чтобы не протекали детали реализации/БД.
            AppError::Database(err) => {
                tracing::error!(error = ?err, "ошибка базы данных");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "внутренняя ошибка сервера".to_string(),
                )
            }
            AppError::Internal(err) => {
                tracing::error!(error = ?err, "внутренняя ошибка");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "внутренняя ошибка сервера".to_string(),
                )
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
