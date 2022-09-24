CREATE TABLE IF NOT EXISTS log_index
(
    id UUID PRIMARY KEY,
    "name" VARCHAR UNIQUE
);

CREATE TABLE IF NOT EXISTS log_accesses
(
    id UUID PRIMARY KEY,
    "index" UUID REFERENCES log_index (id),
    "password" VARCHAR
);

CREATE TABLE IF NOT EXISTS log_log
(
    id UUID PRIMARY KEY, 
    "index" UUID REFERENCES log_index (id),
    data JSONB
);