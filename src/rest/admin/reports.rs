#[cfg(not(debug_assertions))]
use actix_web::error::ErrorUnauthorized;

use actix_web::{get, post, web, HttpRequest};

use crate::rest::schemes::report::Report;
use crate::rest::{error::Result, schemes::rvec::RVec, Database};

use crate::model::log_report as LogReport;

use super::admin_auth;

pub fn routing(cfg: &mut web::ServiceConfig) {}

#[get("reports/")]
async fn get_report(req: HttpRequest, db: Database) -> Result<RVec<Report>> {
    admin_auth(&req, &db).await?;
    Ok(Vec::new().into())
}
