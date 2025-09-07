CREATE EXTENSION IF NOT EXISTS pgcrypto;

DELETE FROM users;
COMMIT;

INSERT INTO users (id, name)
VALUES
    (1, 'Alice Dev'),
    (2, 'Bob Dev'),
    (3, 'Charlie Dev')
ON CONFLICT DO NOTHING;
