use actix_web::body::EitherBody;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use serde_json::json;
use std::rc::Rc;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddlewareMiddleware<S, B>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware {
            service: Rc::new(service),
            _phantom: std::marker::PhantomData,
        })
    }
}

pub struct AuthMiddlewareMiddleware<S, B> {
    service: Rc<S>,
    _phantom: std::marker::PhantomData<B>, // Add this line
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareMiddleware<S, B>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Perform your authentication logic here
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_value) = auth_header {
            // Add your token validation logic here
            if auth_value == "Bearer mysecrettoken" {
                let fut = self.service.call(req);
                return Box::pin(async move {
                    let res = fut.await?.map_into_left_body();
                    Ok(res)
                });
            }
        }

        Box::pin(async move {
            let (req, _pl) = req.into_parts();
            let res = HttpResponse::Unauthorized()
                .json(json!({ "message": "Not authorized" }))
                .map_into_right_body();
            Ok(ServiceResponse::new(req, res))
        })
    }
}
