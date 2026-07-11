//! Общее состояние приложения, доступное всем хендлерам через `State<AppState>`.

use std::sync::Arc;

use sqlx::PgPool;

use crate::shared::config::Config;

/// Разделяемое состояние. Клонируется дёшево: `PgPool` — это Arc внутри,
/// `Config` обёрнут в `Arc`. axum требует, чтобы состояние было `Clone`.
#[derive(Clone)]
pub struct AppState {
    /// Пул соединений с PostgreSQL.
    pub db: PgPool,
    /// Конфигурация приложения (только чтение). Доступна всем хендлерам;
    /// пока ни один срез её не читает — глушим предупреждение.
    #[allow(dead_code)]
    pub config: Arc<Config>,
}
