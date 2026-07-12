//! Конфигурация приложения. Читается один раз при старте из переменных окружения.
//!
//! Локально переменные подхватываются из файла `.env` (см. `main.rs`), в проде —
//! приходят из окружения контейнера. Код при этом одинаковый.

use std::net::SocketAddr;

use anyhow::Context;

/// Настройки приложения.
#[derive(Debug, Clone)]
pub struct Config {
    /// Адрес, на котором слушает HTTP-сервер. env `SERVER_ADDR`, дефолт `0.0.0.0:8080`.
    pub server_addr: SocketAddr,
    /// Строка подключения к PostgreSQL. env `DATABASE_URL` — обязательна.
    pub database_url: String,
    /// Уровень логирования по умолчанию. env `LOG_LEVEL`, дефолт `info`.
    pub log_level: String,
    /// Разрешённые CORS-origins. env `CORS_ALLOWED_ORIGINS` (список через запятую).
    /// В деве дефолт — типичные порты фронта; в проде задаётся окружением деплоя.
    pub cors_allowed_origins: Vec<String>,
}

impl Config {
    /// Читает конфигурацию из переменных окружения.
    ///
    /// Возвращает ошибку с понятным описанием, если обязательная `DATABASE_URL`
    /// не задана или `SERVER_ADDR` имеет неверный формат.
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url =
            std::env::var("DATABASE_URL").context("переменная окружения DATABASE_URL не задана")?;

        let server_addr = std::env::var("SERVER_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:8080".to_string())
            .parse()
            .context("SERVER_ADDR: неверный формат адреса (пример: 0.0.0.0:8080)")?;

        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

        // Список разрешённых origins через запятую. Дефолт — порты фронта в разработке.
        let cors_allowed_origins = std::env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173,http://localhost:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(Self {
            server_addr,
            database_url,
            log_level,
            cors_allowed_origins,
        })
    }
}
