use actix_web::{body::EitherBody, web::Json, Responder};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod response;
pub mod rvec;
pub mod report;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub(crate) struct Success {
    message: String,
}

impl From<String> for Success {
    fn from(s: String) -> Self {
        Success { message: s }
    }
}

impl From<&str> for Success {
    fn from(s: &str) -> Self {
        Success { message: s.into() }
    }
}

impl From<Success> for Json<Success> {
    fn from(s: Success) -> Self {
        Json(s)
    }
}

impl Responder for Success {
    type Body = EitherBody<String>;
    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        Json::respond_to(self.into(), req)
    }
}

impl Default for Success {
    fn default() -> Self {
        Success {
            message: "Success".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub(crate) struct NamedRequest {
    pub name: String,
}
