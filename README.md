# rust_axum_template

Стартовый шаблон веб-API на Rust: **axum** + **sqlx/PostgreSQL**, организованный по
**Vertical Slice Architecture**. Дизайн — в
[docs/superpowers/specs/2026-07-11-rust-axum-vsa-starter-design.md](docs/superpowers/specs/2026-07-11-rust-axum-vsa-starter-design.md).

## Быстрый старт

```bash
# 1. Настройки (один раз)
cp .env.example .env          # PowerShell: Copy-Item .env.example .env

# 2. Поднять базу данных
docker compose up -d

# 3. Запустить сервис (миграции применятся автоматически)
cargo run

# 4. Проверить (на Windows используй 127.0.0.1, а не localhost — см. примечание ниже)
curl http://127.0.0.1:8080/health
# → {"status":"ok","database":"up"}
```

> **Примечание (Windows):** сервер слушает `0.0.0.0` (IPv4). На Windows `localhost`
> часто резолвится в IPv6 (`::1`), поэтому для локальной проверки надёжнее
> обращаться к `127.0.0.1`. В контейнере/на Linux это не важно.

## Команды

| Команда | Что делает |
|---|---|
| `docker compose up -d` | поднять PostgreSQL в контейнере |
| `docker compose down` | остановить базу (данные сохранятся) |
| `docker compose down -v` | снести базу вместе с данными |
| `cargo run` | запустить сервис (накатив миграции) |
| `cargo watch -x check` | автопроверка кода при каждом сохранении |
| `cargo nextest run` | прогнать тесты (требуется поднятая база) |
| `cargo deny check` | проверка зависимостей: уязвимости и лицензии |
| `cargo clippy` | линтер: подсказки по улучшению кода |
| `cargo fmt` | форматирование кода |

## Структура (Vertical Slice)

```
src/
  main.rs            — точка входа: собрать и запустить
  lib.rs             — экспорт модулей (для тестов и бинарника)
  app.rs             — сборка Router из всех срезов + middleware
  shared/            — сквозное: config, error, state, db
  features/          — вертикальные срезы (по одной фиче)
    health/          — GET /health (пример среза)
migrations/          — SQL-миграции (применяются при старте)
```

Новая фича = новая папка в `features/`, экспортирующая свой `router()`,
который подключается в `app.rs` через `.merge(...)`.

## Переменные окружения

См. `.env.example`. Ключевая — `DATABASE_URL` (обязательна).

## API-коллекция (Bruno)

В папке `bruno/` — коллекция [Bruno](https://www.usebruno.com/): open-source API-клиент,
который хранит запросы как файлы прямо в репозитории (в отличие от облачного Postman).
Коллекция версионируется в git и шарится вместе с кодом.

- Открой папку `bruno/` в Bruno как коллекцию (Open Collection).
- Выбери окружение **Local** (`baseUrl = http://127.0.0.1:8080`).
- Готовый запрос **Health** дёргает `GET /health`.
- Новые запросы, созданные в Bruno, появятся как `.bru` файлы в `bruno/` и попадут в git.
