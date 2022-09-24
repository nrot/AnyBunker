ALTER TABLE log_structs
    DROP CONSTRAINT log_structs_index_fkey,
    ALTER COLUMN "index" TYPE VARCHAR; 
update log_structs set "index" = ( select "name" from log_index where log_index.id::text = "index");
ALTER TABLE log_structs
    ALTER COLUMN "index" SET NOT NULL,
    ADD CONSTRAINT  log_structs_index_fkey FOREIGN KEY ("index") REFERENCES log_index ("name");