//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "log_accesses")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub index: Option<Uuid>,
    pub token: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::log_index::Entity",
        from = "Column::Index",
        to = "super::log_index::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    LogIndex,
}

impl Related<super::log_index::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LogIndex.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
