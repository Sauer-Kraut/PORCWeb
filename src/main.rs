use async_std::fs;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_web::http;
use core::panic;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::{self};
use std::sync::mpsc;
use actix_files::Files;
use colored::Colorize;
use std::process;
use std::default;
use std::mem::size_of_val;
use std::ops::{Add, Mul, Sub};
use flume;
use tokio;
use std::sync::{Arc};
use tokio::sync::RwLock;
use rand::Rng;






// retruns the html site we want to serve
async fn index() -> impl Responder {
    println!("\nYay, we got a request!");
    HttpResponse::Ok().body(fs::read_to_string("static/index.html").await.unwrap())
}







// This function handels PUT requests! This basically gets called whenever the client wants some data
// It retruns a PullReqeustSendPackage struct, which then gets send to the client

async fn pull_request(info: web::Json<PullReqeustRecvPackage>, appstate: web::Data<AppState>) -> impl Responder {
    println!("\n\n\n\n{} {} \ndescription: {}", "Received Pull Request:".bold().cyan(), info.title.bold().italic().cyan(), info.description.italic());

    let (data_sender, data_receiver) = mpsc::channel();
    let (error_sender, error_receiver) = mpsc::channel();




    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {

        if false {
            error_sender.send("error".to_owned());
        }

    }).await.unwrap();
    println!("arrived behind async thread");




    // println!("trying to receive");
    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); err},
        Err(_) => "No Error detected".to_string(),
    };

    let data = data_receiver.recv().unwrap();

    HttpResponse::Ok().json(PullReqeustSendPackage {
        title: "Server Respons".to_string(),
        description: "results calculated with given data".to_string(),
        data
    })
}









#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                .allowed_origin("https://PORC.mywire.org") // Update with your frontend's origin
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![
                    http::header::AUTHORIZATION,
                    http::header::ACCEPT,
                    http::header::ORIGIN,
                    http::header::CONTENT_TYPE,
                ])
                .supports_credentials()
                .max_age(3600))
            .app_data(web::Data::new(AppState {
                // here we can safe data we want to keep across requests
                // It gets delivered to the functions we call as an additional argument
            }))
            .service(Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").to(index))
            .service(web::resource("/index").to(index))
            .service(web::resource("/api/pull-request")
            .route(web::put().to(pull_request)))
    })
    .bind("0.0.0.0:8081")? // Caddy forwarts requests to our URL to the local port 8081
    .run()
    .await
}














#[derive(Debug, Deserialize, Serialize)]
pub struct PullReqeustRecvPackage {
    pub title: String,
    pub description: String,      //Karina says this would be beneficial
    pub data: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PullReqeustSendPackage {
    pub title: String,
    pub description: String,      //Karina says this would be beneficial
    pub data: String
}





struct AppState {
    // render_recources: Arc<RwLock<RenderRecources>>,
    // compute_recources: Arc<RwLock<ComputeGroupingRecources>>
}