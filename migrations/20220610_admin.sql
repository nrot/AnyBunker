CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE log_admin_user
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token VARCHAR(32) UNIQUE DEFAULT md5(random()::text),
    name VARCHAR(16)
);

CREATE TABLE log_admin_ticks
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "interval" INTERVAL NOT NULL,
    "name" VARCHAR UNIQUE
);