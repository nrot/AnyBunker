-- TODO: THIS NEED CREATE FROM SUPERUSER;

SELECT 'THIS NEED CREATE FROM SUPERUSER';
-- exit

CREATE EXTENSION IF NOT EXISTS plpython3u;
CREATE EXTENSION IF NOT EXISTS jsonb_plpython3u;
-- CREATE OR REPLACE FUNCTION jsonb_recursive(val jsonb) RETURNS TABLE(KEY VARCHAR)
-- transform for type jsonb
-- as $$

-- def f(d):
--     re = set()
--     if type(d) is dict:
--         for k in d:
--             v = d[k]
--             if type(v) is dict or type(v) is list:
--                 for ki in f(v):
--                     re.add(f"{k}.{ki}")
--             else:
--                 re.add(k)
--     elif type(d) is list:
--         for v in d:
--             if type(v) is dict or type(v) is list:
--                 for k in f(v):
--                     re.add(k)
--     else:
--         re.add(str(type(d)))
--     return re

-- return f(val)

-- $$ language plpython3u;

select DISTINCT jsonb_recursive(data) from log_log;
select * from log_log;

-- TODO: REWRITE TO RUST FUNCITON