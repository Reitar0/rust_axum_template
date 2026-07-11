//! Инициализация пула соединений с БД и применение миграций.

use anyhow::Context;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// Создаёт пул соединений с PostgreSQL.
pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .context("не удалось подключиться к PostgreSQL")
}

/// Применяет все ещё не применённые миграции из каталога `migrations/`.
///
/// Миграции встраиваются в бинарник на этапе компиляции (`migrate!`),
/// поэтому для сборки нужна лишь папка `migrations/`, а не живая БД.
pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("не удалось применить миграции")?;
    Ok(())
}
