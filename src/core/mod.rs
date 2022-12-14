use actix_web::web;
use chrono::Utc;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use sea_query::{PostgresQueryBuilder, Query};

use crate::model::log_log as LogLog;
use crate::utils::with_schema;

pub mod error;

pub type Database = web::Data<DatabaseConnection>;

pub async fn insert_message(
    db: &DatabaseConnection,
    index: &str,
    data: serde_json::Value,
) -> error::Result<()> {
    let tr = with_schema(index);
    let date: DateTimeWithTimeZone = Utc::now().into();

    let query = Query::insert()
        .into_table(tr)
        .columns(vec![
            LogLog::Column::Data,
            LogLog::Column::Timestamp,
        ])
        .values([
            data.into(),
            date.into(),
        ])?
        .to_owned();

    let q = query.build(PostgresQueryBuilder);

    match db
        .execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            &q.0,
            q.1,
        ))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
