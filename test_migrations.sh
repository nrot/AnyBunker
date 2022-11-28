#!/bin/bash

source .env

echo "$DB"

PGPASSWORD=$PASSWORD

PORT=5453
TMP_DB=tmp_database
SOCKET=$(pwd)/$TMP_DB/sockets

echo "Creating test database"
mkdir $TMP_DB/
initdb $TMP_DB/

echo "DB Socket: $SOCKET"
mkdir $SOCKET

echo "Start postgres server"
pg_ctl -D $TMP_DB/ -l $TMP_DB/logs -m fast -o "-F -i -p $PORT -k $SOCKET/" start 

echo "Create user"
psql -h $SOCKET -p $PORT -d postgres -c "CREATE ROLE $USER WITH LOGIN SUPERUSER INHERIT CREATEDB NOCREATEROLE PASSWORD '$PASSWORD';"

DATABASE_URL=postgres://$USER:$PASSWORD@localhost:$PORT/$DB 

echo "Create database"
sqlx database create -D $DATABASE_URL
echo "Run migrations"
sqlx migrate run -D $DATABASE_URL

echo "Check some DB tables"
psql -A -h $SOCKET -p $PORT -d $DB -c "SELECT * FROM pg_catalog.pg_tables where tablename ilike 'erp_%' limit 10;"

echo "Generate rust structs"
sea-orm-cli generate entity --with-serde both --with-copy-enums -u $DATABASE_URL -s public -o ./src/model/ --ignore-tables wb_model_nomenclatures 

echo "Stop postgres server"
pg_ctl -D $TMP_DB/ -l $TMP_DB/logs -m fast -o "-F -p $PORT -k $SOCKET/" stop
echo "Cleanup"
rm -rf $TMP_DB
