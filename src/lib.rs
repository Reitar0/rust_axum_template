//! Библиотечная часть проекта.
//!
//! Архитектура — **Domain-oriented Vertical Slice**:
//! - `shared/`  — сквозное (config, error, db, state), доступно всем;
//! - `system/`  — системные эндпоинты (health), не бизнес-логика;
//! - `domains/` — бизнес-домены; каждый домен изолирован и не использует код
//!   другого домена (проверяется тестом `tests/architecture.rs`).
//!
//! `main.rs` — тонкая обёртка, которая собирает и запускает приложение.

pub mod app;
pub mod domains;
pub mod graphql;
pub mod openapi;
pub mod shared;
pub mod system;
