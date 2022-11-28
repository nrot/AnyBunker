-- CREATE TABLE IF NOT EXISTS log_accesses
-- (
--     id UUID PRIMARY KEY,
--     "index" UUID REFERENCES log_index (id),
--     "password" VARCHAR
-- );

ALTER TABLE log_accesses RENAME COLUMN "password" TO "token";
ALTER TABLE log_accesses ALTER COLUMN "token" TYPE VARCHAR(32);