use std::{collections::HashMap, sync::Arc, fmt::Display};
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use utoipa::ToSchema;
use crate::model;
use sea_orm::entity::prelude::Uuid;

pub type HashTable = HashMap<String, Vec<String>>;

#[derive(Debug, Clone)]
pub struct AccessHashMap(Arc<RwLock<HashTable>>);

impl From<HashTable> for AccessHashMap {
    fn from(h: HashTable) -> Self {
        AccessHashMap(Arc::new(RwLock::new(h)))
    }
}

impl AccessHashMap {
    pub fn inner(&self) -> &Arc<RwLock<HashTable>> {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct LogMessage {
    pub index: String,
    pub token: String,
    pub data: serde_json::Value,
}

impl From<LogMessage> for model::log_log::Model{
    fn from(l: LogMessage) -> Self {
        let time: chrono::DateTime<chrono::FixedOffset> = chrono::Local::now().into();
        model::log_log::Model{
            id: Uuid::nil(),
            // index: l.index,
            timestamp: Some(time),
            data: l.data
        }
    }
}

impl Display for LogMessage{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index: {} data {}", self.index, self.data)
    }
}