use chrono::{DateTime, Utc};
use futures::join;
use sqlx::*;
use sqlx::types::Json;

use crate::liberary::{account_lib::{account::{account::Account, account_customisation::AccountCustomisation, discord_user::{self, DiscordUser}, user_data::{radar_chart::{self, RadarChart}, shiftstone::Shiftstone, account_stats::AccountStats}}, availability::availability::{Availability, DailyRepetitionConfig, Repetition}, match_event::match_event::{MatchEvent, MatchStatus}, schedule::{self, schedule::Schedule, storage::get_schedule::get_schedule}}, util::functions::build_query::{ArgumentType, build_query}};




// Ugliest pieve of shit code Ive ever written, but I do not have the time to refactor to the SQL function architecture Ive been working on. Surely I will find time to fix this soon


#[derive(sqlx::FromRow, serde::Deserialize, Debug, Clone)]
struct SubQueryStructAccount {
    id: String,
    username: String,
    discriminator: i32,
    avatar: Option<String>,
    email: Option<String>,
    schedule_note: Option<String>
}

impl TryInto<DiscordUser> for SubQueryStructAccount {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_into(self) -> Result<DiscordUser, Self::Error> {
        let res = discord_user::DiscordUser {
            id: self.id.parse()?,
            username: self.username,
            discriminator: self.discriminator.to_string(),
            avatar: self.avatar,
            email: self.email,
        };
        Ok(res)
    }
}

#[derive(sqlx::FromRow, serde::Deserialize, Debug, Clone)]
struct SubQueryStructStats {
    wins: u32,
    matches: u32,
    win_ratio: Option<f64>,
    global_rank: Option<u32>,
    division_rank: Option<u32>,
    _pose_stats: Option<serde_json::Value>,
}

impl Into<AccountStats> for SubQueryStructStats {
    fn into(self) -> AccountStats {
        AccountStats {
            wins: self.wins,
            matches: self.matches,
            win_ratio: self.win_ratio,
            global_rank: self.global_rank,
            division_rank: self.division_rank,
        }
    }
}

#[derive(sqlx::FromRow, serde::Deserialize, Debug, Clone)]
struct SubQueryStructRadarChart {
    mobility: f32,
    weight: f32,
    aggresiveness: f32,
    range: f32,
    reactivity: f32,
}

impl TryInto<RadarChart> for SubQueryStructRadarChart {
    type Error = Box<dyn std::error::Error + Send + Sync>;
    
    fn try_into(self) -> Result<RadarChart, Self::Error> {
        RadarChart::new(vec![
            ("mobility".to_string(), self.mobility),
            ("weight".to_string(), self.weight),
            ("aggresiveness".to_string(), self.aggresiveness),
            ("range".to_string(), self.range),
            ("reactivity".to_string(), self.reactivity),
        ], None).map_err(|e| e.into())
    }
}

#[derive(sqlx::FromRow, serde::Deserialize, Debug, Clone)]
struct SubQueryStructCustomisation {
    radar_chart: Option<SubQueryStructRadarChart>,
    stone_1: Option<String>,
    stone_2: Option<String>,
    badge_1: Option<u64>,
    badge_2: Option<u64>,
    badge_3: Option<u64>,
    unlocked_badges: Vec<u64>,
    region: Option<String>,
    bp: u32,
    banner: Option<String>,
}

impl TryInto<AccountCustomisation> for SubQueryStructCustomisation {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_into(self) -> Result<AccountCustomisation, Self::Error> {
        let res = AccountCustomisation {
            radar_chart: self.radar_chart.map(|r|r.try_into()).transpose()?,
            shiftstones: (self.stone_1.map(|s| Shiftstone::from(s)), self.stone_2.map(|s| Shiftstone::from(s))),
            badges: vec![self.badge_1, self.badge_2, self.badge_3]
                .into_iter()
                .flatten()
                .map(|badge| badge as u64)
                .collect(),
            unlocked_badges: self.unlocked_badges,
            region: self.region,
            bp: self.bp,
            banner: self.banner,
        };
        Ok(res)
    }
}

#[derive(sqlx::FromRow, Debug)]
struct QueryStruct {
    account_info: Json<SubQueryStructAccount>,
    stats: Json<SubQueryStructStats>,
    customisation: Json<SubQueryStructCustomisation>
}

pub async fn get_global_leaderboard(pool: PgPool) -> Result<Vec<Account>, Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/account_lib/account/storage/queries/get_global_ranking.sql";
    let query = build_query(query_path, vec![])?;

    let res = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_all(&pool).await?;

    let accounts = res.iter().map(|res| {
        let discord_user = res.account_info.0.clone().try_into()?;
        let user_stats: AccountStats = res.stats.0.clone().into();
        let customisation: Option<AccountCustomisation> = Some(res.customisation.0.clone().try_into()?);

        let account = Account {
            user_info: discord_user,
            schedule: None,
            customisation,
            stats: user_stats,
        };
        Ok(account)
    }).collect::<Result<Vec<_>, Box<dyn std::error::Error + Send + Sync>>>()?;
    
    Ok(accounts)
}