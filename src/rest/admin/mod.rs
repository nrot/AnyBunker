use actix_web::error::ErrorNotFound;

#[cfg(not(debug_assertions))]
use actix_web::error::ErrorUnauthorized;

use actix_web::{get, post, web, HttpRequest};

use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{
    ActiveValue, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, FromQueryResult,
    QueryFilter, QueryOrder, QuerySelect, Statement,
};
use sea_query::Expr;
use sea_query::{ColumnDef, PostgresQueryBuilder, Query, Table};
use serde::Deserialize;

use ts_rs::TS;

use crate::event;
#[cfg(not(debug_assertions))]
use crate::model::log_admin_user as LogAdminUser;
use crate::model::log_index as LogIndex;
use crate::model::log_log as LogLog;
use crate::model::log_structs as LogStructs;
use crate::rest::error::Result;
use crate::rest::schemes::{self, rvec::RVec};
use crate::utils::with_schema;

use crate::rest::Database;

mod reports;

pub fn routing(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .service(create_index)
            .service(search_list)
            .service(get_index)
            .service(run_reindex)
            .service(autocomplete_index)
            .configure(reports::routing),
    );
}

#[cfg(not(debug_assertions))]
#[inline]
async fn admin_auth(req: &HttpRequest, db: &DatabaseConnection) -> Result<()> {
    let h = req.headers();
    let token = h
        .get("Authorization")
        .ok_or_else(|| ErrorUnauthorized("Not have Authorization"))?
        .to_str()
        .map_err(|_| ErrorUnauthorized("Can`t parse string"))?;

    LogAdminUser::Entity::find()
        .filter(LogAdminUser::Column::Token.eq(token))
        .one(db)
        .await?
        .map(|_| ())
        .ok_or_else(|| ErrorUnauthorized("Token not found").into())
}

#[cfg(debug_assertions)]
#[inline(always)]
async fn admin_auth(_: &HttpRequest, _: &DatabaseConnection) -> Result<()> {
    Ok(())
}

#[post("/index")]
async fn create_index(
    req: HttpRequest,
    db: Database,
    data: web::Json<schemes::NamedRequest>,
) -> Result<schemes::Success> {
    admin_auth(&req, db.as_ref()).await?;

    let index = data.into_inner().name;
    let d = LogIndex::ActiveModel {
        name: ActiveValue::Set(Some(index.clone())),
        id: ActiveValue::Set(uuid::Uuid::new_v4()),
    };

    LogIndex::Entity::insert(d).exec(db.as_ref()).await?;

    let t = Table::create()
        .table(with_schema(&index))
        .col(
            ColumnDef::new(LogLog::Column::Id)
                .uuid()
                .extra("DEFAULT uuid_generate_v4()".into())
                .not_null(),
        )
        .col(
            ColumnDef::new(LogLog::Column::Timestamp)
                .timestamp_with_time_zone()
                .extra("DEFAULT NOW()".into())
                .not_null(),
        )
        .col(
            ColumnDef::new(LogLog::Column::Data)
                .json_binary()
                .not_null(),
        )
        .to_owned();
    db.execute(sea_orm::Statement::from_string(
        db.get_database_backend(),
        t.build(PostgresQueryBuilder),
    ))
    .await
    .map(|_| schemes::Success::default())
    .map_err(|e| e.into())
}

#[get("/index")]
async fn get_index(req: HttpRequest, db: Database) -> Result<RVec<String>> {
    admin_auth(&req, db.as_ref()).await?;

    LogIndex::Entity::find()
        .all(db.as_ref())
        .await
        .map(|r| {
            r.into_iter()
                .filter_map(|m| m.name)
                .collect::<Vec<String>>()
                .into()
        })
        .map_err(|e| e.into())
}

#[post("/{index}/reindex")]
async fn run_reindex(
    req: HttpRequest,
    db: Database,
    index: web::Path<String>,
    e: web::Data<event::SenderEvents>,
) -> Result<schemes::Success> {
    admin_auth(&req, db.as_ref()).await?;

    let _ = LogIndex::Entity::find()
        .filter(LogIndex::Column::Name.eq(index.clone()))
        .one(db.as_ref())
        .await?
        .ok_or_else(|| ErrorNotFound("Index not founded"))?;

    e.send(event::Events::ReIndex {
        index: index.into_inner(),
    })?;

    Ok("Job reindex start".into())
}

#[derive(Deserialize, TS)]
#[ts(export)]
struct SearchQuery {
    time_start: DateTimeWithTimeZone,
    time_end: Option<DateTimeWithTimeZone>,
    take: Option<u64>,
    skip: Option<u64>,
    query: Option<String>,
}

//select * from log_log where "data" @@ '$.message.pixels.name == "goolge_pixel"';
//select * from log_log where "data" @@ '$.message.pixels.name == "goolge_pixel" && $.origin == "dreamwhite.ru"';
#[get("/{index}/search")]
async fn search_list(
    req: HttpRequest,
    db: Database,
    index: web::Path<String>,
    q: web::Query<SearchQuery>,
) -> Result<RVec<schemes::log_response::LogResponse>> {
    admin_auth(&req, db.as_ref()).await?;

    let ind = index.into_inner();

    LogIndex::Entity::find()
        .filter(LogIndex::Column::Name.eq(ind.clone()))
        .one(db.as_ref())
        .await?
        .ok_or_else(|| ErrorNotFound("Index not founded"))?;

    let mut pre_query = Query::select();
    pre_query
        .from(with_schema(&ind))
        .columns(vec![
            LogLog::Column::Id,
            LogLog::Column::Timestamp,
            LogLog::Column::Data,
        ])
        .and_where(Expr::col(LogLog::Column::Timestamp).gt(q.time_start.date_naive()))
        .and_where(Expr::cust(&match &q.query {
            Some(v) => format!(r#""data" @@ '{}'"#, v),
            None => String::from("1 = 1"),
        }))
        .limit(q.take.unwrap_or(10))
        .offset(q.skip.unwrap_or(0));
    if let Some(te) = q.time_end {
        pre_query.and_where(Expr::col(LogLog::Column::Timestamp).lt(te.date_naive()));
    }
    let query = pre_query.build(PostgresQueryBuilder);

    let res = db
        .query_all(Statement::from_sql_and_values(
            db.get_database_backend(),
            &query.0,
            query.1,
        ))
        .await?;
    let mut to_response = Vec::new();
    for r in res {
        to_response.push(schemes::log_response::LogResponse::from(
            LogLog::Model::from_query_result(&r, "")?,
        )); //
    }
    Ok(to_response.into())
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
struct AutocompleteQuery {
    s: String,
    limit: Option<u64>,
}

#[get("/{index}/autocomplete")]
async fn autocomplete_index(
    req: HttpRequest,
    db: Database,
    index: web::Path<String>,
    q: web::Query<AutocompleteQuery>,
) -> Result<RVec<String>> {
    admin_auth(&req, db.as_ref()).await?;

    let ind = index.into_inner();

    let _ = LogIndex::Entity::find()
        .filter(LogIndex::Column::Name.eq(ind.clone()))
        .one(db.as_ref())
        .await?
        .ok_or_else(|| ErrorNotFound("Index not founded"))?;

    let s = q.s.replace("$.", "");

    Ok(LogStructs::Entity::find()
        .filter(LogStructs::Column::Index.eq(ind))
        .filter(LogStructs::Column::Path.starts_with(&s))
        .order_by_asc(LogStructs::Column::Path)
        .limit(q.limit.unwrap_or(10))
        .all(db.as_ref())
        .await?
        .into_iter()
        .map(|m| m.path)
        .collect::<Vec<_>>()
        .into())
}
