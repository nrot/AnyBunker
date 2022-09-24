-- CREATE TABLE log_admin_ticks
-- (
--     id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
--     "interval" INT NOT NULL,
--     "name" VARCHAR UNIQUE
-- );

INSERT INTO log_admin_ticks ("interval", "name") VALUES( 1, 'System 1 second ticker' );