use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;
use std::sync::Arc;

mod systems;
mod reports;

pub type ReportPull = Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>;

pub async fn init(pull: ReportPull, db: DatabaseConnection, dp: dptree::di::DependencyMap){
    let reports = tokio::spawn(reports::create_report(pull.clone(), db.clone(), dp.clone()));
    let mut p = pull.write().await;
    p.push(tokio::spawn(systems::reindex(dp.clone())));
    p.push(reports);
}
