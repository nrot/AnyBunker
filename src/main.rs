use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;

mod credentials;
mod event;
mod handlers;
mod init;
mod model;
mod rest;
mod schemes;
mod utils;
mod rpc;
mod core;

#[cfg(unix)]
async fn system_signals_stops() {
    let c = Box::pin(tokio::signal::ctrl_c());
    let mut t = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).unwrap();
    let _ = futures::future::select(Box::pin(t.recv()), c).await;
}

#[cfg(windows)]
async fn system_signals_stops() {
    tokio::signal::ctrl_c().await.unwrap();
}

#[actix_web::main]
async fn main() {
    simple_logger::init_with_level(if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Info
    })
    .unwrap();

    let e = dotenv().ok();
    log::info!("Env: {:?}", e);


    log::info!("Server start");

    let pg = init::connect_postgres().await;

    let mut dp = dptree::di::DependencyMap::new();

    utils::generate_styles();

    let tickers = init::run_tickers(pg.clone()).await;
    let (tx, mut _rx) = init::create_bus();
    let access_map: schemes::AccessHashMap = init::access_rules(pg.clone()).await;

    dp.insert(access_map.clone());
    dp.insert(tx.clone());
    dp.insert(pg.clone());

    let handelrs = init::run_handlers(pg.clone(), dp.clone()).await;

    let db = pg.clone();
    let hs = HttpServer::new(move || {
        App::new()
            .wrap(rest::cors())
            .app_data(web::Data::new(access_map.clone()))
            // .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(tx.clone()))
            .configure(rest::routing)
    })
    .bind(credentials::httpserver_bind_uri())
    .unwrap()
    .shutdown_timeout(1);

    let s = tokio::spawn(hs.run());
    let rpc_server = tokio::spawn(rpc::run(pg.clone()));
 
    system_signals_stops().await;

    rpc_server.abort();
    s.abort();
    tickers.iter().for_each(|t| {
        t.abort();
    });
    handelrs.write().await.iter().for_each(|t| {
        t.abort();
    });
}
