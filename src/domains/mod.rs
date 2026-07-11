//! Бизнес-домены (Domain-oriented Vertical Slice).
//!
//! Каждый домен — подпапка здесь (`users/`, `orders/`, `auth/`...) со слоями:
//!   mod.rs         — объявляет слои-подмодули и экспортирует `router()`
//!   dto.rs         — объекты запроса/ответа (API-контракт)
//!   handler.rs     — HTTP-хендлеры (тонкий слой axum: extractors ↔ usecase)
//!   usecase.rs     — бизнес-логика (юзкейсы; одно действие = одна функция)
//!   repository.rs  — доступ к БД (SQL через sqlx)
//!   model.rs       — доменные модели/сущности
//!
//! ПРАВИЛО ИЗОЛЯЦИИ: домен НЕ использует код другого домена (никаких
//! `crate::domains::<другой>`). Общее — только через `crate::shared`.
//! Проверяется автотестом `tests/architecture.rs`.
//!
//! Скелет `mod.rs` домена:
//!   mod dto;
//!   mod handler;
//!   mod usecase;
//!   mod repository;
//!   mod model;
//!
//!   use axum::Router;
//!   use axum::routing::post;
//!   use crate::shared::state::AppState;
//!
//!   pub fn router() -> Router<AppState> {
//!       Router::new().route("/users", post(handler::register_user))
//!   }

use axum::Router;

use crate::shared::state::AppState;

// Пример (раскомментируй, когда добавишь домен):
// mod users;

/// Собирает роуты всех бизнес-доменов.
pub fn router() -> Router<AppState> {
    Router::new()
    // .merge(users::router())
}
