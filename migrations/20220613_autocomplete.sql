CREATE TABLE IF NOT EXISTS log_structs
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "index" UUID REFERENCES log_index (id),
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    path VARCHAR(256)
);