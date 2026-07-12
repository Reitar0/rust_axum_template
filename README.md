# rust_axum_template

![CI](https://github.com/Reitar0/rust_axum_template/actions/workflows/ci.yml/badge.svg)

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

## Структура (Domain-oriented Vertical Slice)

```
src/
  main.rs            — точка входа: собрать и запустить
  lib.rs             — экспорт модулей (для тестов и бинарника)
  app.rs             — сборка Router из system + domains + middleware
  shared/            — сквозное: config, error, state, db (доступно всем)
  system/            — системные эндпоинты (health) — не бизнес-логика
  domains/           — бизнес-домены; каждый изолирован от других
    <domain>/        — домен со слоями:
      mod.rs         — подмодули + router()
      dto.rs         — объекты запроса/ответа (API-контракт)
      handler.rs     — HTTP-хендлеры (тонкий слой axum)
      usecase.rs     — бизнес-логика (юзкейсы)
      repository.rs  — доступ к БД (SQL)
      model.rs       — доменные модели
migrations/          — SQL-миграции (применяются при старте)
tests/
  architecture.rs    — автотест: домен не использует код чужого домена
```

Новый домен = папка `domains/<domain>/` с `mod.rs` (экспортирует `router()`),
подключается в `domains/mod.rs` через `.merge(...)`. Правило: домен не импортирует
другой домен — только `shared`. Это проверяет `tests/architecture.rs`.

## Переменные окружения

См. `.env.example`. Ключевые:
- `DATABASE_URL` — подключение к БД (обязательна).
- `CORS_ALLOWED_ORIGINS` — разрешённые origins фронта через запятую. В проде задаётся
  окружением деплоя реальным доменом фронта (не `*`, не `localhost`).

## API-коллекция (Bruno)

В папке `bruno/` — коллекция [Bruno](https://www.usebruno.com/): open-source API-клиент,
который хранит запросы как файлы прямо в репозитории (в отличие от облачного Postman).
Коллекция версионируется в git и шарится вместе с кодом.

- Открой папку `bruno/` в Bruno как коллекцию (Open Collection).
- Выбери окружение **Local** (`baseUrl = http://127.0.0.1:8080`).
- Готовый запрос **Health** дёргает `GET /health`.
- Новые запросы, созданные в Bruno, появятся как `.bru` файлы в `bruno/` и попадут в git.

## Несколько проектов одновременно

Каждый проект из шаблона использует **свою** БД (отдельный Docker-том), но хост-порты
по умолчанию одинаковые. Чтобы запустить несколько проектов разом, задай в `.env`
каждого уникальные порты:

| Проект | DB_PORT | SERVER_ADDR | порт в DATABASE_URL |
|---|---|---|---|
| A | 5432 | 0.0.0.0:8080 | 5432 |
| B | 5433 | 0.0.0.0:8081 | 5433 |

`DB_PORT` подставляется в `compose.yaml` (проброс порта БД), `SERVER_ADDR` — порт
сервера, а порт в `DATABASE_URL` должен совпадать с `DB_PORT`.

## Команды через just

Установка (один раз): `cargo install just`. Затем:

| Команда | Что делает |
|---|---|
| `just dev` | поднять БД (дождавшись готовности) и запустить сервис |
| `just watch` | то же + автоперезапуск при изменениях (cargo-watch) |
| `just test` | поднять БД и прогнать тесты (cargo-nextest) |
| `just seed` | наполнить БД тестовыми данными из `seeds/dev.sql` |
| `just stop` | остановить БД (данные сохраняются) |
| `just clean` | убрать БД вместе с данными |
| `just check` | быстрая проверка кода |
| `just` | список всех команд |

БД остаётся поднятой между запусками сервиса (быстрый рестарт). Гаси её
командой `just stop`, когда закончишь работу.

## API-документация (Swagger) и GraphQL

При запущенном сервисе доступны:

| URL | Что это |
|---|---|
| `http://127.0.0.1:8080/api` | Swagger UI — интерактивная документация REST |
| `http://127.0.0.1:8080/api-docs/openapi.json` | OpenAPI 3.1 JSON (генерируется из аннотаций utoipa) |
| `http://127.0.0.1:8080/graphql` | GraphQL: POST — запросы, GET — песочница GraphiQL |

**Добавить REST-эндпоинт в Swagger:** пометь хендлер `#[utoipa::path(...)]`, его DTO —
`#[derive(ToSchema)]`, и зарегистрируй в `src/openapi.rs` (`paths(...)`, `components(schemas(...))`).

**Добавить GraphQL-поле:** новый метод в `impl Query` (`src/graphql/query.rs`). Пул БД
доступен резолверам через `ctx.data::<sqlx::PgPool>()`.

## Инструменты разработки

`rustup` доустановит нужный toolchain + rustfmt/clippy/rust-src/llvm-tools автоматически
(из `rust-toolchain.toml`). Cargo-утилиты поставь одной командой (`--locked` — nextest требует его):

    cargo install --locked just cargo-nextest cargo-watch cargo-audit cargo-edit cargo-llvm-cov cargo-deny cargo-outdated cargo-machete

(быстрее — `cargo binstall ...` готовыми бинарниками, если установлен `cargo-binstall`.)
Либо после установки `just` — доставить всё разом через `just setup`.

| Утилита | Зачем |
|---|---|
| `just` | task-runner (`just dev`, `just test`, …) |
| `cargo-nextest` | быстрый тест-раннер (`just test`, CI) |
| `cargo-watch` | автоперезапуск (`just watch`) |
| `cargo-deny` | аудит зависимостей: уязвимости + лицензии |
| `cargo-audit` | аудит уязвимостей (RustSec) |
| `cargo-edit` | `cargo upgrade` / `set-version` (`add`/`rm` уже встроены в cargo) |
| `cargo-llvm-cov` | покрытие кода тестами |
| `cargo-outdated` | устаревшие зависимости |
| `cargo-machete` | неиспользуемые зависимости |
