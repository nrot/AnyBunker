mod admin;
mod auth;
mod schemes;

use crate::{core::{self, Database, error}, rest::schemes::Success, credentials};

use actix_cors::Cors;
use actix_web::{get, post, web, HttpRequest};



use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::event;

use super::schemes as gschemes;


pub fn routing(cfg: &mut web::ServiceConfig) {
    cfg.service(insert_json)
        .service(ping);
        // .service(SwaggerUi::new("/swagger/{_:.*}").url("insert-json", InsertJson::openapi()));
    admin::routing(cfg);
}

#[cfg(not(debug_assertions))]
pub fn cors() -> Cors {
    use actix_web::http;

    let mut c = Cors::default();

    for origin in credentials::allowed_origin(){
        c = c.allowed_origin(&origin);
    }

    c.allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(60 * 60 * 24)
}
#[cfg(debug_assertions)]
pub fn cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(60 * 60 * 24)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        insert_json,
    ),
    components(
        schemas(gschemes::LogMessage)
    ),
    tags(
        (name="logs", description="log inserting")
    )
)]
struct InsertJson;

#[utoipa::path(context_path="/", responses((status=200, description="Вставка данных по индексу и паролю", body=Success)))]
#[post("/insert/json")]
async fn insert_json(
    _req: HttpRequest,
    data: web::Json<gschemes::LogMessage>,
    ah: web::Data<gschemes::AccessHashMap>,
    db: Database,
    _e: web::Data<event::SenderEvents>,
) -> error::Result<schemes::Success> {
    auth::auth_client(ah.as_ref(), &data.index, &data.token).await?;

    let index = data.0.index.clone();

    core::insert_message(&db, &index, data.data.to_owned()).await.map(|_|Success::from(""))
}

#[get("/ping")]
async fn ping(_req: HttpRequest) -> String {
    "pong".into()
}
