ALTER TABLE log_log
    ALTER COLUMN id SET DEFAULT gen_random_uuid();