use std::fmt::{Debug, Display};

use actix_web::{body::EitherBody, web::Json, Responder, ResponseError};
use serde::Serialize;

#[allow(type_alias_bounds)]
pub type Result<T: Responder> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Http(actix_web::error::Error),
    DB(sea_orm::error::DbErr),
    SyncBr(String),
    Json(serde_json::Error),
    Sql(sea_query::error::Error),
}

impl Serialize for Error {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl Display for Error {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h: String = match self {
            Error::Http(_) => "Http error: ".into(),
            Error::DB(_) => "DB error: ".into(),
            Error::SyncBr(_) => "Sync Broadcast error: ".into(),
            Error::Json(e) => {
                format!(
                    "Json error: {:?} in line {} column {}: ",
                    e.classify(),
                    e.line(),
                    e.column()
                )
            }
            Error::Sql(_) => "Sql build error:".into(),
        };
        write!(f, "{}{:?}", h, &self)
    }
}

impl From<actix_web::error::Error> for Error {
    #[inline(always)]
    fn from(e: actix_web::error::Error) -> Self {
        Error::Http(e)
    }
}

impl From<sea_orm::error::DbErr> for Error {
    #[inline(always)]
    fn from(e: sea_orm::error::DbErr) -> Self {
        Error::DB(e)
    }
}

impl From<serde_json::Error> for Error {
    #[inline(always)]
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

impl From<sea_query::error::Error> for Error {
    #[inline(always)]
    fn from(e: sea_query::error::Error) -> Self {
        Error::Sql(e)
    }
}

impl<T: ToString> From<tokio::sync::broadcast::error::SendError<T>> for Error {
    #[inline(always)]
    fn from(e: tokio::sync::broadcast::error::SendError<T>) -> Self {
        Error::SyncBr(e.to_string())
    }
}

impl From<Error> for Json<Error> {
    #[inline(always)]
    fn from(e: Error) -> Self {
        Json(e)
    }
}

impl Responder for Error {
    type Body = EitherBody<String>;
    #[inline(always)]
    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        Json::respond_to(self.into(), req)
    }
}

impl ResponseError for Error {
    #[inline(always)]
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::Http(h) => h.error_response().status(),
            Error::SyncBr(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            _ => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
}
