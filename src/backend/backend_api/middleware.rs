use std::future::{ready, Ready};
use std::thread;
use std::time::Duration;

use actix_http::body::MessageBody;
use actix_web::body::{to_bytes, BoxBody};
use actix_web::{dev::*, Error, HttpResponse};
use actix_web::dev::ServiceRequest;
use colored::Colorize;
use futures::future::LocalBoxFuture;
use serde_json::to_vec;

use crate::backend::backend_api::server_error::ServerError;


pub struct ServerMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ServerMiddleware 
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static + actix_web::body::MessageBody
{
    type Response = ServiceResponse<BoxBody>;
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
    B: 'static + actix_web::body::MessageBody
{
    type Error = Error;
    type Response = ServiceResponse<BoxBody>;
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
            // let _ = thread::sleep(Duration::from_secs(5)); // FOR TESTING ONLY
            let res = fut.await;

            let duration = start.elapsed();
            // there has to be a better way of doing this
            println!("{} {} {} {} {}{}{}{}", "Finished".cyan(), method.cyan().bold(), "at".cyan(), path.cyan().bold(), "(", "delta: ", format!("{duration:?}").bold(), ")");

            match res {
                Ok(r) => {

                    let status = r.status();

                    // deconstructing response to get the body as bytes
                    let (req_parts, req_body) = r.into_parts();
                    let (res_parts, res_body) = req_body.into_parts();
                    let body_bytes = to_bytes(res_body).await
                        .map_err(|_e| {
                            actix_web::error::ErrorInternalServerError("Error converting response body to bytes")
                        })?;

                    // println!("{}", String::from_utf8(body_bytes.to_vec()).unwrap_or("".to_string()).yellow().bold());

                    if !status.is_success() {
                        eprintln!("{} {}\n", "finished with status code:".red(), status.as_str().red().bold());
                        // eprintln!("{}", ServerError::from(body_bytes.to_vec()).red().bold());
                    } else {
                        println!("\n")
                    }

                    let new_body = BoxBody::new(body_bytes);



                    // reconstructing the response in order to return it
                    let mut builder = HttpResponse::build(status);

                    for (key, value) in res_parts.headers().iter() {
                        builder.insert_header((key.clone(), value.clone()));
                    }

                    let new_res = builder.body(new_body);
                    
                    let new_response = ServiceResponse::new(req_parts, new_res);
                    
                    return Ok(new_response)
                },
                Err(e) => {
                    eprintln!("{}{}{}\n{}","Error occured at ".red(), path.red().bold(), ":".red(), e.to_string().red());
                    return Err(e)
                },
            }
        })
    }
}