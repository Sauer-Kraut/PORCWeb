use std::collections::HashMap;
use chrono::{DateTime, Utc};
use sqlx::*;

use crate::liberary::{matchplan_lib::{division::division::Division, matchplan::matchplan::MatchPlan, matchplan_match::matchplan_match::Match, player::player::Player}, util::functions::build_query::*};

#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    player_1_tag: String,
    player_1_id: String,
    player_1_score: Option<i16>,

    player_2_tag: String,
    player_2_id: String,
    player_2_score: Option<i16>,

    division_name: String,
    division_order: i16,

    season_end_timestamp: DateTime<Utc>,
    season_pause_end_timestamp: DateTime<Utc>
}

pub async fn get_matchplan(season_name: String, pool: PgPool) -> Result<MatchPlan, Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/matchplan_lib/matchplan/storage/queries/get_matchplan.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(season_name.clone())
    ])?;

    let rows = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_all(&pool)
    .await?;

    let mut players = Vec::new();
    let mut divisions: HashMap<String, Division> = HashMap::new();

    let end_timestamp = rows[0].season_end_timestamp.timestamp() as u64;
    let pause_end_timestamp = rows[0].season_pause_end_timestamp.timestamp() as u64;

    for row in rows {
        let player_1 = Player {
            id: row.player_1_id.parse()?,
            tag: row.player_1_tag.clone(),
            division: row.division_name.clone(),
        };
        let player_2 = Player {
            id: row.player_2_id.parse()?,
            tag: row.player_2_tag.clone(),
            division: row.division_name.clone(),
        };

        let match_info = Match {
            p1: player_1.clone(),
            p2: player_2.clone(),
            p1score: row.player_1_score.map(|score| score as usize),
            p2score: row.player_2_score.map(|score| score as usize),
        };

        let match_key = match_info.generate_key();

        match divisions.get_mut(&row.division_name) {
            Some(division) => {
                division.matches.insert(match_key, match_info);
                if division.players.contains(&player_1) == false {
                    division.players.push(player_1.clone());
                }
                if division.players.contains(&player_2) == false {
                    division.players.push(player_2.clone());
                }
            },
            None => {
                let mut new_division = Division {
                    name: row.division_name.clone(),
                    order: row.division_order as usize,
                    matches: HashMap::new(),
                    players: vec![player_1.clone(), player_2.clone()],
                };
                new_division.matches.insert(match_key, match_info);
                divisions.insert(row.division_name.clone(), new_division);
            }
        };
    }

    let mut divisions_vec: Vec<Division> = Vec::new();
    for division in divisions.values() {
        players.extend(division.players.clone());
        divisions_vec.push(division.clone());
    }

    let matchplan = MatchPlan {
        divisions: divisions_vec,
        players,
        season: season_name,
        end_timestamp: end_timestamp,
        pause_end_timestamp: pause_end_timestamp
    };

    // println!("{:?}", matchplan);

    Ok(matchplan)
}