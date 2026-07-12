# Команды для разработки. Запуск: `just <команда>` (список: `just`).
#
# Кроссплатформенный шелл: на Windows — PowerShell, на Unix — sh (по умолчанию).
set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]

# Показать список команд (по умолчанию)
default:
    @just --list

# Установить cargo-утилиты проекта (--locked: nextest требует его; just ставится отдельно).
setup:
    cargo install --locked cargo-nextest cargo-watch cargo-audit cargo-edit cargo-llvm-cov cargo-deny cargo-outdated cargo-machete

# Поднять БД (ждёт готовности) и запустить сервис; БД не гасится при выходе
dev:
    docker compose up -d --wait
    cargo run

# Поднять БД и автоперезапуск сервиса при изменениях (нужен cargo-watch).
watch:
    docker compose up -d --wait
    cargo watch -x run

# Поднять БД и прогнать тесты (нужен cargo-nextest).
test:
    docker compose up -d --wait
    cargo nextest run

# Наполнить БД тестовыми данными из seeds/dev.sql (psql внутри контейнера).
[windows]
seed:
    docker compose up -d --wait
    Get-Content seeds/dev.sql -Raw | docker compose exec -T db psql -U app -d app

[unix]
seed:
    docker compose up -d --wait
    docker compose exec -T db psql -U app -d app < seeds/dev.sql

# Остановить БД (данные в volume сохраняются, рестарт быстрый).
stop:
    docker compose stop

# Полностью убрать БД вместе с данными (volume удаляется).
clean:
    docker compose down -v

# Быстрая проверка кода без сборки бинарника.
check:
    cargo check
