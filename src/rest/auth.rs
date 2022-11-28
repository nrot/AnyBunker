use std::{future::{ready, Ready}, rc::Rc, cell::RefCell};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, web::{Json}, FromRequest,
};
use futures::future::LocalBoxFuture;

use crate::schemes;

///Функция простой авторизации пользователя.
#[inline(always)]
pub async fn auth_client(ah: &schemes::AccessHashMap, index: &String, password: &String)->Result<(), Error>{
    if let Some(p) = ah.inner().read().await.get(index){
        if !p.contains(password){
            Err(actix_web::error::ErrorUnauthorized(""))
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}





// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[deprecated]
#[derive(Default)]
pub struct TokenAuth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
#[allow(deprecated)]
impl<S, B> Transform<S, ServiceRequest> for TokenAuth
where
    S: 'static + Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TokenAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TokenAuthMiddleware(Rc::new(RefCell::new(service)))))
    }
}

#[deprecated]
#[derive(Debug)]
pub struct TokenAuthMiddleware<S>(Rc<RefCell<S>>);

#[allow(deprecated)]
impl<S, B> Service<ServiceRequest> for TokenAuthMiddleware<S>
where
    S: 'static + Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    #[inline]
    fn poll_ready(
        &self,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx).map_err(::core::convert::Into::into)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let hm = req
            .app_data::<schemes::AccessHashMap>()
            .expect("Can`t find AccessHashMap").clone();

        let svc = self.0.clone();
        
        Box::pin(async move {
            if req.content_type().to_lowercase() == "application/json" {
                let (r, p) = req.parts_mut();
                if let Ok(data) = <Json<schemes::LogMessage> as FromRequest>::from_request(r, p).await{
                    let h = hm.inner().read().await;
                    if let Some(p) = h.get(&data.index) {
                        if !p.contains(&data.token) {
                            return Err(actix_web::error::ErrorUnauthorized(""));
                        }
                    }
                } else {
                    return Err(actix_web::error::ErrorBadRequest("Request payload or Json parse error"));
                }
            } else {
                return Err(actix_web::error::ErrorUnsupportedMediaType("Auth request json schema"));
            }
            
            let fut = svc.call(req);
            let res = fut.await?;
            Ok(res)
        })
    }
}
