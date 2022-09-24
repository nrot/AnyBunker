ALTER TABLE log_structs 
    ALTER COLUMN "index" SET NOT NULL,
    ALTER COLUMN path SET NOT NULL,
    ADD CONSTRAINT struct_pk PRIMARY KEY ("index", path),
    DROP COLUMN id;
