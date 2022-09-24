//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "log_index")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::log_structs::Entity")]
    LogStructs,
    #[sea_orm(has_many = "super::log_report::Entity")]
    LogReport,
    #[sea_orm(has_many = "super::log_accesses::Entity")]
    LogAccesses,
}

impl Related<super::log_structs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LogStructs.def()
    }
}

impl Related<super::log_report::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LogReport.def()
    }
}

impl Related<super::log_accesses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LogAccesses.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}