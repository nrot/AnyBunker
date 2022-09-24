ALTER TABLE public.log_report DROP COLUMN columns;
ALTER TABLE public.log_report ADD COLUMN columns JSONB;