use std::future::{ready, Ready};

use actix_web::{dev::*, Error};
use actix_web::dev::ServiceRequest;
use colored::Colorize;
use futures::future::LocalBoxFuture;


pub struct ServerMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ServerMiddleware 
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ServerMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ServerMiddlewareService {service}))
    }
}


pub struct ServerMiddlewareService<S> {
    service: S
}

impl<S, B> Service<ServiceRequest> for ServerMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static
{
    type Error = Error;
    type Response = ServiceResponse<B>;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }
    
    fn call(&self, req: ServiceRequest) -> Self::Future {

        let start = std::time::Instant::now();
        let method = req.method().to_string();
        let path = req.path().to_owned();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            let duration = start.elapsed();
            // there has to be a better way of doing this
            println!("\n{} {} {} {} {}{}{}{}", "Received".cyan(), method.cyan().bold(), "at".cyan(), path.cyan().bold(), "(", "delta: ", format!("{duration:?}").bold(), ")");

            return Ok(res);
        })
    }
}