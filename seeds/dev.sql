-- Seed-данные для локальной разработки.
-- Запуск: `just seed`. Идемпотентно (ON CONFLICT) — можно гонять повторно.
-- ВАЖНО: только фейковые данные для разработки. Не копия прода, не PII.

-- Пример: наполнение служебной таблицы (замени на свои доменные таблицы).
INSERT INTO _app_meta (key, value) VALUES
    ('seeded', 'true'),
    ('env', 'development')
ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value;

-- Когда добавишь доменные таблицы (например users) — вставляй тестовые данные сюда:
-- INSERT INTO users (id, email, name) VALUES
--     (gen_random_uuid(), 'alice@example.com', 'Алиса'),
--     (gen_random_uuid(), 'bob@example.com',   'Боб')
-- ON CONFLICT DO NOTHING;
