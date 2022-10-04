use crate::model;
use actix_web::{body::EitherBody, web::Json, Responder};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub enum ReportType {
    Email,
    File,
}

impl From<model::sea_orm_active_enums::ReportSendType> for ReportType {
    fn from(m: model::sea_orm_active_enums::ReportSendType) -> Self {
        match m {
            model::sea_orm_active_enums::ReportSendType::Email => ReportType::Email,
            model::sea_orm_active_enums::ReportSendType::File => ReportType::File,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct Report {
    #[ts(type="string")]
    pub id: Uuid,
    pub cron: String,
    pub interval: Option<i32>,
    pub query: String,
    pub name: String,
    #[ts(type="string")]
    pub index: Uuid,
    pub send_type: ReportType,
    pub send_to: String,
    pub columns: Vec<String>,
}

impl From<model::log_report::Model> for Report {
    fn from(m: model::log_report::Model) -> Self {
        Report {
            id: m.id,
            cron: m.cron,
            interval: m.interval,
            query: m.query,
            name: m.name,
            index: m.index.unwrap_or_default(),
            send_type: m
                .send_type
                .unwrap_or( model::sea_orm_active_enums::ReportSendType::File)
                .into(),
            send_to: m.send_to.unwrap_or_default(),
            columns: serde_json::from_value(m.columns.unwrap()).unwrap(),
        }
    }
}

impl From<Report> for Json<Report> {
    fn from(l: Report) -> Self {
        Json(l)
    }
}

impl Responder for Report {
    type Body = EitherBody<String>;
    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        Json::respond_to(self.into(), req)
    }
}
