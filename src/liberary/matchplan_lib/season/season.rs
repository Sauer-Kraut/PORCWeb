use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::liberary::{account_lib::signup::storage::get_signups::get_signups, matchplan_lib::{matchplan::storage::matchplan_get::get_matchplan, matchplan_blueprint::matchplan_blueprint::PlanBlueprint}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Season {
    pub name: String,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub pause_end_timestamp: u64
}

impl Season {

    pub async fn get_blueprint(&self, pool: PgPool) -> Result<PlanBlueprint, Box<dyn std::error::Error + Send + Sync>> {

        let matchplan = get_matchplan(self.name.clone(), pool.clone()).await?;
        let signups = get_signups(self.start_timestamp, Some(self.pause_end_timestamp), pool).await?;

        let blueprint = matchplan.generate_blueprint(signups).await;
        Ok(blueprint)
    }
}