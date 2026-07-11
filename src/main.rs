//! Точка входа. Задача `main` — «собрать» приложение из частей и запустить.
//! Вся логика — в библиотечном крейте (`lib.rs`): слой `shared/` и срезы `features/`.

use std::sync::Arc;

use anyhow::Context;
use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use rust_axum_template::app;
use rust_axum_template::shared::config::Config;
use rust_axum_template::shared::db;
use rust_axum_template::shared::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Локально подгружаем .env (в проде переменные приходят из окружения).
    // Отсутствие файла — не ошибка.
    let _ = dotenvy::dotenv();

    let config = Config::from_env()?;
    init_tracing(&config.log_level);

    // Подключаемся к БД и применяем миграции при старте.
    let db = db::create_pool(&config.database_url).await?;
    db::run_migrations(&db).await?;

    let addr = config.server_addr;
    let state = AppState {
        db,
        config: Arc::new(config),
    };

    let router = app::build_router(state);

    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("не удалось привязаться к адресу {addr}"))?;
    tracing::info!("сервер слушает на http://{addr}");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("ошибка HTTP-сервера")?;

    Ok(())
}

/// Инициализирует структурированное логирование.
///
/// Уровень берётся из `RUST_LOG`, а если она не задана — из `log_level` конфига.
fn init_tracing(log_level: &str) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Ждёт сигнал завершения (Ctrl+C или SIGTERM) для корректной остановки.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("не удалось установить обработчик Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("не удалось установить обработчик SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("получен сигнал завершения, останавливаемся...");
}
