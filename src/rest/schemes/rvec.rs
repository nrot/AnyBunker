use actix_web::{body::EitherBody, web::Json, Responder};
use serde::Serialize;



#[derive(Debug, Serialize)]
pub struct  RVec<T: Responder + Serialize>(Vec<T>);

impl<T: Serialize + Responder> From<RVec<T>> for Json<RVec<T>>{
    fn from(v: RVec<T>) -> Self {
        Json(v)
    }
}

impl<T: Responder + Serialize> From<Vec<T>> for RVec<T>{
    fn from(v: Vec<T>) -> Self {
        RVec(v)
    }
}

impl<T: Serialize + Responder> Responder for RVec<T>{
    type Body = EitherBody<String>;
    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        Json::respond_to(self.into(), req)
    }
}