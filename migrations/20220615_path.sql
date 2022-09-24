-- CREATE EXTENSION IF NOT EXISTS plpython3u;
-- create EXTENSION IF NOT EXISTS jsonb_plpython3u;
-- create or replace function jsonb_recursive(val jsonb) returns table(key varchar)
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

-- select DISTINCT jsonb_recursive(data) from log_log;
-- select * from log_log