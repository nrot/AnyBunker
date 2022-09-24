use crate::model;
use actix_web::{body::EitherBody, web::Json, Responder};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub(crate) struct LogResponse {
    // pub index: Option<String>,
    pub timestamp: DateTime<FixedOffset>,
    #[ts(type="any")]
    pub data: sea_orm::JsonValue,
}

impl From<model::log_log::Model> for LogResponse {
    fn from(m: model::log_log::Model) -> Self {
        LogResponse {
            // index: Some(m.index),
            timestamp: m.timestamp.unwrap_or_else(|| {
                let time: chrono::DateTime<chrono::FixedOffset> = chrono::Local::now().into();
                time
            }),
            // data: utils::json_html_syntect(serde_json::to_string_pretty(&m.data).unwrap()),
            data: m.data,
        }
    }
}

impl From<LogResponse> for Json<LogResponse>{
    fn from(l: LogResponse) -> Self {
        Json(l)
    }
}

impl Responder for LogResponse {
    type Body = EitherBody<String>;
    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        Json::respond_to(self.into(), req)
    }
}