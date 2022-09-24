ALTER TABLE log_log
    DROP CONSTRAINT log_log_index_fkey,
    ALTER COLUMN "index" TYPE VARCHAR; 
update log_log set "index" = ( select "name" from log_index where log_index.id::text = "index");
ALTER TABLE log_log
    ALTER COLUMN "index" SET NOT NULL,
    ADD CONSTRAINT  log_log_index_fkey FOREIGN KEY ("index") REFERENCES log_index ("name");