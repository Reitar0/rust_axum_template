# Multi-stage сборка: собираем в полном rust-образе, а в финальный кладём
# только готовый бинарник — образ получается компактным.

# --- Стадия сборки ---
FROM rust:1-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release
# Примечание: для ускорения повторных сборок можно внедрить cargo-chef
# (кэширование слоя зависимостей отдельно от кода) — но и без него всё работает.

# --- Финальная стадия ---
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# ca-certificates нужны для TLS-подключений к внешним БД (managed PostgreSQL).
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust_axum_template /usr/local/bin/app
EXPOSE 8080
CMD ["app"]
