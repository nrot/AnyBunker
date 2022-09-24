use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status, transport::Server};

pub mod rpc;

use crate::core;

pub struct RpcLogServer{
    db: DatabaseConnection,
}

type RequestResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl rpc::logger_server::Logger for RpcLogServer{
    async fn send_message(&self, request: Request<rpc::LogMessage>)->RequestResult<rpc::ResultMessage>{
        let m = request.get_ref();
        log::info!("rpc request: {:?}", m);
        let v = if let Ok(v) = serde_json::from_str(m.data.as_str()) {
            v
        } else{
            return Err(Status::invalid_argument("Can`t convert json from data"))
        };
        if let Err(e) = core::insert_message(&self.db, &m.index, v).await{
            return Err(Status::internal("Can`t insert value"));
        };
        Ok(Response::new(rpc::ResultMessage{
            ok: true
        }))
    }
}

pub async fn run(db: DatabaseConnection){
    log::info!("Rpc server starting");
    //TODO: Move to env
    let addr = "[::1]:50051".parse().unwrap();
    let server = RpcLogServer{
        db
    };

    let svc = rpc::logger_server::LoggerServer::new(server);

    Server::builder().add_service(svc).serve(addr).await.unwrap();
}