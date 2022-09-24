use std::sync::Arc;

use dptree::di::DependencySupplier;
use sea_orm::{
   EntityTrait, QueryFilter, TransactionTrait,
};
use sea_orm::{ColumnTrait, ConnectionTrait, DatabaseConnection, FromQueryResult, Value};

use crate::{event, credentials};
use crate::model::log_index as LogIndex;
use crate::model::log_structs as LogStruct;

#[derive(FromQueryResult)]
struct ReindexResult {
    pub path: String,
}

pub async fn reindex(dp: dptree::di::DependencyMap) {
    let s: Arc<event::SenderEvents> = dp.get();
    let mut r = s.as_ref().subscribe();
    let db: Arc<DatabaseConnection> = dp.get();
    loop {
        if let event::Events::ReIndex { index } = r.recv().await.unwrap() {
            let ind = match LogIndex::Entity::find()
                .filter(LogIndex::Column::Name.eq(index.clone()))
                .one(db.as_ref())
                .await
            {
                Ok(r) => match r {
                    Some(r) => r,
                    None => {
                        log::info!("Can`t find index: {}", index);
                        continue;
                    }
                },
                Err(e) => {
                    log::error!("Can`t find index: {}", e);
                    continue;
                }
            };
            let paths: Vec<ReindexResult> =
                match ReindexResult::find_by_statement(sea_orm::Statement::from_sql_and_values(
                    db.get_database_backend(),
                    &format!(r#"SELECT DISTINCT jsonb_recursive("data") AS path FROM "{}"."{}" 
                    WHERE ( 
                        (SELECT count("timestamp") FROM log_structs WHERE "index"=$1 ) = 0 
                        OR timestamp > (SELECT "timestamp" FROM log_structs WHERE "index" = $1 ORDER BY "timestamp" DESC LIMIT 1 )
                    )"#, credentials::postgres_schema(), index),
                    vec![Value::from(index.clone())],
                ))
                .all(db.as_ref())
                .await
                {
                    Ok(r) => {
                        log::info!("Success reindex {}", index);
                        r
                    }
                    Err(e) => {
                        log::error!("Error reindex {:?}", e);
                        continue;
                    }
                };
            
            
            let tx = match db.as_ref().begin().await{
                Ok(t)=>{
                    log::info!("Transaction reindex start");
                    t
                }, 
                Err(e)=>{
                    log::error!("Error transaction reindex start: {}",e);
                    continue;
                }
            };

            for p in paths {
                let _ = LogStruct::Entity::find().from_raw_sql(sea_orm::Statement::from_sql_and_values(
                    tx.get_database_backend(),
                    r#"INSERT INTO log_structs ("index", path) VALUES ($1, $2) ON CONFLICT ("index", path) DO UPDATE SET timestamp = CURRENT_TIMESTAMP"#,
                    vec![Value::from(ind.name.clone().unwrap()), Value::from(p.path)]
                )).one(&tx)
                .await
                .map_err(|e| {
                    log::error!("Update error: {}", e);
                });
            }
            let _ = tx.commit().await.map_err(|e|{
                log::error!("Reindex transaction failed: {}", e);
            });
        }
    }
}
