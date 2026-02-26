use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::AppState;
use crate::liberary::discord_lib::video_reference::video_reference::VideoReference;




#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespPackage {
    pub videos: Vec<VideoReference>,
}



// GET Request to receive all posted vods
pub async fn get_discord_vods_reqeust(query: web::Query<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let vods = appstate.discord_vods.read().await.clone();

    Ok(HttpResponse::Ok().json(RespPackage {
        videos: vods,
    }))
}