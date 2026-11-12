use chrono::{DateTime, Utc};
use sqlx::*;
use sqlx::types::Json;

use crate::liberary::{account_lib::{account::{account::Account, account_customisation::AccountCustomisation, discord_user::{self, DiscordUser}, user_data::{account_stats::AccountStats, radar_chart::RadarChart, shiftstone::Shiftstone}}, availability::availability::{Availability, DailyRepetitionConfig, Repetition}, availability_exception::availability_exception::AvailabilityException, match_event::match_event::{MatchEvent, MatchStatus}, schedule::schedule::Schedule}, util::functions::build_query::{ArgumentType, build_query}};




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
struct SubQueryStructMatchEvent {
    id: i64,
    event_id: Option<String>,
    season: String,
    status_code: i16,
    challenger_id: String,
    opponent_id: String,
    start_timestamp: DateTime<Utc>,
}

impl TryInto<MatchEvent> for SubQueryStructMatchEvent {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_into(self) -> Result<MatchEvent, Self::Error> {
        let res = MatchEvent {
            id: Some(self.id as i32),
            event_id: self.event_id,
            season: self.season,
            challenger_id: self.challenger_id.parse()?,
            opponent_id: self.opponent_id.parse()?,
            start_timestamp: self.start_timestamp.timestamp() as u64,
            status: MatchStatus::from_status_code(self.status_code)?,
        };
        Ok(res)
    }
}

#[derive(sqlx::FromRow, serde::Deserialize, Debug, Clone)]
struct SubQueryStructAvailability {
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,

    repetition_type: i32,

    config_mon: bool,
    config_tue: bool,
    config_wed: bool,
    config_thu: bool,
    config_fri: bool,
    config_sat: bool,
    config_sun: bool
}

impl TryInto<Availability> for SubQueryStructAvailability {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_into(self) -> Result<Availability, Self::Error> {
        let repetition = Repetition::from_type_code(self.repetition_type as i16)?;

        let repetition_config = match &repetition {
            Repetition::Daily => Some(DailyRepetitionConfig {
                monday: self.config_mon,
                tuesday: self.config_tue,
                wednesday: self.config_wed,
                thursday: self.config_thu,
                friday: self.config_fri,
                saturday: self.config_sat,
                sunday: self.config_sun,
            }),
            _ => None
        };

        Ok(Availability {
            start_timestamp: self.start_date.timestamp() as u64,
            end_timestamp: self.end_date.timestamp() as u64,
            repetition,
            repetition_config,
        })
    }
}

#[derive(sqlx::FromRow, serde::Deserialize, Debug, Clone)]
struct SubQueryStructAvailabilityException {
    start_timestamp: u64,
    end_timestamp: u64,
}

impl TryInto<AvailabilityException> for SubQueryStructAvailabilityException {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_into(self) -> Result<AvailabilityException, Self::Error> {
        Ok(AvailabilityException {
            start_timestamp: self.start_timestamp,
            end_timestamp: self.end_timestamp,
        })
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
            pose_stats: self._pose_stats.map(|value| {
                value.as_object().map(|obj| {
                    obj.iter().filter_map(|(key, val)| {
                        val.as_f64().map(|f| (key.clone(), f as f32))
                    }).collect()
                }).unwrap_or_default()
            }),
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
    availabilities: Json<Vec<SubQueryStructAvailability>>,
    availability_exceptions: Json<Vec<SubQueryStructAvailabilityException>>,
    match_events: Json<Vec<SubQueryStructMatchEvent>>,
    stats: Json<SubQueryStructStats>,
    customisation: Option<Json<SubQueryStructCustomisation>>
}

pub async fn get_accounts_full(account_ids: Vec<String>, pool: PgPool) -> Result<Vec<(Account, Vec<MatchEvent>)>, Box<dyn std::error::Error + Send + Sync>> {
        if account_ids.is_empty() {
                return Ok(vec![]);
        }

        let query_path = "src/liberary/account_lib/account/storage/queries/get_account_full.sql";
        let query = build_query(query_path, vec![
                ArgumentType::StringArray(account_ids),
        ])?;

        let rows = sqlx::query_as::<Postgres, QueryStruct>(&query)
                .fetch_all(&pool)
                .await?;

        let mut accounts = Vec::with_capacity(rows.len());

        for res in rows {
            let availabilities = match res.availabilities.iter().map(|av| {av.clone().try_into()}).collect::<Result<Vec<_>, _>>() {
                Ok(value) => value,
                Err(_) => continue,
            };
            let availability_exceptions = match res.availability_exceptions.iter().map(|av| {av.clone().try_into()}).collect::<Result<Vec<_>, _>>() {
                Ok(value) => value,
                Err(_) => continue,
            };
            let match_events = match res.match_events.iter().map(|ev| ev.clone().try_into()).collect::<Result<Vec<MatchEvent>, _>>() {
                Ok(value) => value,
                Err(_) => continue,
            };
            let schedule = Schedule { availabilities, availability_exceptions, matches: match_events.clone().iter().filter_map(|ev| ev.id).collect(), note: res.account_info.schedule_note.clone().unwrap_or("".to_string()) };
            let discord_user = match res.account_info.0.clone().try_into() {
                Ok(value) => value,
                Err(_) => continue,
            };

            let user_stats: AccountStats = res.stats.0.into();

            let customisation: Option<AccountCustomisation> = match res.customisation {
                Some(value) => match value.0.try_into() {
                    Ok(parsed) => Some(parsed),
                    Err(_) => continue,
                },
                None => None,
            };

            let account = Account {
                user_info: discord_user,
                schedule: Some(schedule),
                customisation,
                stats: user_stats,
            };

            accounts.push((account, match_events));
        }

        Ok(accounts)
}

pub async fn get_account_full(account_id: String, pool: PgPool) -> Result<(Account, Vec<MatchEvent>), Box<dyn std::error::Error + Send + Sync>> {
        get_accounts_full(vec![account_id], pool)
                .await?
                .into_iter()
                .next()
                .ok_or_else(|| Box::new(sqlx::Error::RowNotFound) as Box<dyn std::error::Error + Send + Sync>)
}