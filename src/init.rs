use std::collections::HashMap;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, EntityTrait};

use tokio::sync::RwLock;
use std::sync::Arc;

use crate::{credentials, model, schemes, handlers::{self, ReportPull}, event};

async fn sqlx_migrate() {
    let p = sqlx::postgres::PgPoolOptions::new()
        .max_connections(credentials::postgres_connections())
        .connect(&credentials::postgres_uri())
        .await
        .unwrap();
    sqlx::migrate!().run(&p).await.unwrap();
    p.close().await;
}

pub async fn connect_postgres() -> DatabaseConnection {
    sqlx_migrate().await;

    let u = credentials::postgres_uri();
    let mut c = ConnectOptions::new(u);
        c.max_connections(credentials::postgres_connections());
    Database::connect(c).await.unwrap()
}

pub async fn run_tickers(
    db: DatabaseConnection,
) -> Vec<tokio::task::JoinHandle<()>> {
    let t = model::log_admin_ticks::Entity::find().all(&db).await.unwrap();
    t.into_iter().map(|t|{
        tokio::spawn(async move {
            loop{
                tokio::time::sleep(std::time::Duration::from_secs(t.interval as u64)).await;
            }
        })
    }).collect()
}

pub async fn run_handlers(db: DatabaseConnection, data: dptree::di::DependencyMap) -> ReportPull {
    let tasks = Arc::new(RwLock::new(Vec::new()));
    handlers::init(tasks.clone(), db, data).await;
    tasks
}

#[inline(always)]
pub fn create_bus() -> (
    event::SenderEvents,
    event::ReceiverEvents,
) {
    tokio::sync::broadcast::channel(credentials::bus_size())
}

pub async fn access_rules(
    db: DatabaseConnection,
) -> schemes::AccessHashMap {
    let mut hm: HashMap<String, Vec<String>> = HashMap::new();

    let ac = model::log_accesses::Entity::find().all(&db).await.unwrap();

    for a in ac.iter() {
        let li = model::log_index::Entity::find_by_id(a.index.unwrap())
            .one(&db)
            .await
            .unwrap().unwrap();
        match hm.get_mut(li.name.as_ref().unwrap()) {
            Some(p) => p.push(a.password.clone().unwrap()),
            None => {
                hm.insert(li.name.unwrap(), vec![a.password.clone().unwrap()]);
            }
        }
    }
    log::debug!("AccessHashMap: {:?}", hm);
    hm.into()
}
