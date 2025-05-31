use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError; 
use crate::liberary::account_lib::signup::signup::SignUpInfo;
use crate::liberary::account_lib::signup::storage::get_signups::get_signups;
use crate::liberary::matchplan_lib::season::storage::get_season::get_season;
use crate::AppState;



#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
    pub season: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RespPackage {
    pub signups: Vec<SignUpInfo>,
}


// GET Request to retrieve all sign ups of a specified season
pub async fn get_sign_ups_request(query: web::Query<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let season_name = match &query.season {
        Some(v) => v.clone(),
        None => {
            let season_opt = appstate.season.read().await.clone();
            match season_opt {
                Some(season) => season.name.clone(),
                None => {
                    return Err(ServerError::Other("No current season found".to_owned().into()));
                }
            }
        }
    };

    let (mut start, mut end) = (0, None);

    match get_season(season_name, appstate.pool.clone()).await? {
        Some(v) => {
            start = v.start_timestamp;
            end = Some(v.pause_end_timestamp);
        }
        _ => {}
    };

    let signups = get_signups(start, end, appstate.pool.clone()).await?;

    Ok(HttpResponse::Ok().json(RespPackage {
        signups,
    }))
}
