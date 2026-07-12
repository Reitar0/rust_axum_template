//! Интеграционный тест среза health.
//!
//! Требует поднятую базу данных и заданную `DATABASE_URL`:
//!   docker compose up -d
//! Тест поднимает приложение на случайном свободном порту и проверяет,
//! что `GET /health` возвращает 200 и корректное тело.

use std::sync::Arc;

use rust_axum_template::app;
use rust_axum_template::graphql;
use rust_axum_template::shared::config::Config;
use rust_axum_template::shared::db;
use rust_axum_template::shared::state::AppState;

#[tokio::test]
async fn health_returns_ok() {
    // Берём DATABASE_URL так же, как при обычном запуске (из .env / окружения).
    let _ = dotenvy::dotenv();
    let config = Config::from_env().expect("нужна DATABASE_URL: подними базу и настрой .env");

    let db = db::create_pool(&config.database_url)
        .await
        .expect("подключение к БД (docker compose up -d)");
    db::run_migrations(&db).await.expect("применение миграций");

    let graphql_schema = graphql::build_schema(db.clone());
    let state = AppState {
        db,
        config: Arc::new(config),
        graphql: graphql_schema,
    };
    let router = app::build_router(state);

    // Поднимаем сервер на случайном свободном порту (порт 0 → ОС выберет сама).
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind на свободный порт");
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });

    // Делаем реальный HTTP-запрос к поднятому серверу.
    let resp = reqwest::get(format!("http://{addr}/health"))
        .await
        .expect("запрос к /health");
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.expect("тело как JSON");
    assert_eq!(body["status"], "ok");
    assert_eq!(body["database"], "up");
}
