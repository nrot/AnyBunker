use super::RequestResult;
use crate::model::log_index as LogIndex;
use sea_orm::{DatabaseConnection, EntityTrait};
use tonic::{Response, Status};

mod admin;
mod index;

pub use admin::admin_panel_server::AdminPanelServer;

pub struct AdminRpcServer {
    pub db: DatabaseConnection,
}

#[tonic::async_trait]
impl admin::admin_panel_server::AdminPanel for AdminRpcServer {
    ///
    async fn get_indexes(
        &self,
        request: tonic::Request<index::IndexListRequest>,
    ) -> RequestResult<index::IndexListResponse> {
        match LogIndex::Entity::find().all(&self.db).await {
            Ok(data) => Ok(Response::new(index::IndexListResponse {
                indexes: data
                    .into_iter()
                    .map(|m| index::Index {
                        id: m.id.to_string(),
                        name: m.name.unwrap_or_default(),
                    })
                    .collect(),
            })),
            Err(e) => Err(Status::internal(format!("Database error: {}", e))),
        }
    }
    ///
    async fn search(
        &self,
        request: tonic::Request<index::IndexSearchRequest>,
    ) -> RequestResult<index::IndexSearchResult> {
        Ok(Response::new(index::IndexSearchResult {
            response: Vec::new(),
        }))
    }
}
