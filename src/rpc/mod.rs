use sea_orm::DatabaseConnection;
use tonic::{transport::Server, Request, Response, Status};

mod admin;
mod rpc;

use crate::{core, credentials};

pub struct RpcServer {
    db: DatabaseConnection,
}

type RequestResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl rpc::saver_server::Saver for RpcServer {
    async fn send_message(
        &self,
        request: Request<rpc::Message>,
    ) -> RequestResult<rpc::ResultMessage> {
        let m = request.get_ref();
        log::info!("rpc request: {:?}", m);
        let v = if let Ok(v) = serde_json::from_str(m.data.as_str()) {
            v
        } else {
            return Err(Status::invalid_argument("Can`t convert json from data"));
        };
        if let Err(e) = core::insert_message(&self.db, &m.index, v).await {
            log::error!("Can`t inser value: {}", e);
            return Err(Status::internal("Can`t insert value"));
        };
        Ok(Response::new(rpc::ResultMessage { ok: true }))
    }
}

pub async fn run(db: DatabaseConnection) {
    log::info!("Rpc server starting");
    let addr = credentials::grpcserver_bind_uri().parse().unwrap();
    let server = RpcServer { db: db.clone() };
    let admin = admin::AdminRpcServer { db: db.clone() };

    let svc = rpc::saver_server::SaverServer::new(server);
    let admin_server = admin::AdminPanelServer::new(admin);
    let corsed = tonic_web::GrpcWebLayer::new();


    Server::builder()
        .trace_fn(|_| tracing::info_span!("Grpc request: {}"))
        .accept_http1(true)
        .layer(corsed)
        .add_service(admin_server)
        .add_service(svc)
        .serve(addr)
        .await
        .unwrap();
}
