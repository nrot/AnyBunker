CREATE TYPE "report_send_type" AS ENUM ('file','email');

ALTER TABLE public.log_report ADD COLUMN IF NOT EXISTS send_type report_send_type; 
ALTER TABLE public.log_report ADD COLUMN IF NOT EXISTS send_to VARCHAR;