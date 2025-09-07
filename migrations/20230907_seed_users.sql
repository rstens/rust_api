CREATE EXTENSION IF NOT EXISTS pgcrypto;

DELETE FROM users;
COMMIT;

INSERT INTO users (id, name)
VALUES
    (gen_random_uuid(), 'Alice Dev'),
    (gen_random_uuid(), 'Bob Dev'),
    (gen_random_uuid(), 'Charlie Dev')
ON CONFLICT DO NOTHING;
