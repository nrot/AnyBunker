-- CREATE TABLE log_admin_ticks
-- (
--     id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
--     "interval" INT NOT NULL,
--     "name" VARCHAR UNIQUE
-- );

CREATE TABLE IF NOT EXISTS public.log_report(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    cron VARCHAR NOT NULL,
    "interval" INT,
    query VARCHAR NOT NULL,
    "name" VARCHAR NOT NULL UNIQUE,
    "index" UUID REFERENCES public.log_index (id)    
);