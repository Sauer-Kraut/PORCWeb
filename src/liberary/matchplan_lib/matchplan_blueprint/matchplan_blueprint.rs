use serde::{Deserialize, Serialize};

use crate::liberary::{account_lib::signup::signup::SignUpInfo, matchplan_lib::{division, matchplan::matchplan::MatchPlan}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanBlueprint {
    pub divisions: Vec<DivisionBlueprint>,
    pub players_to_sort: Vec<PlayerBlueprint>,
    pub end_timestamp: Option<u64>,
    pub pause_end_timestamp: Option<u64>,
    pub season: u64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DivisionBlueprint {
    pub name: String,
    pub order: usize,
    pub players: Vec<PlayerBlueprint>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerBlueprint {
    pub tag: String,
    pub id: String
}

impl PartialEq for PlayerBlueprint {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl MatchPlan {

    pub async fn generate_blueprint(&self, signups: Vec<SignUpInfo>) -> PlanBlueprint {

        let mut division_blueprints: Vec<DivisionBlueprint> = Vec::new();

        let mut ordered_divisions = self.divisions.clone();
        ordered_divisions.sort_by(|a, b| a.order.cmp(&b.order));

        let mut players_to_demote = Vec::new();

        for (index, division) in ordered_divisions.iter().enumerate() {

            let mut division_blueprint = DivisionBlueprint {
                name: division.name.clone(),
                order: division.order,
                players: players_to_demote.clone().iter().map(|player: &division::player_performance::PlayerPerformance| PlayerBlueprint{
                    tag: player.player.tag.clone(),
                    id: player.player.id.clone(),
                }).collect()
            };

            let players = division.generate_perfomance().await;

            let (players_to_promote, remaining_players) = players.split_at((players.len() as f32 / 3.0).floor() as usize);
            let (players_to_keep, demoting) = remaining_players.split_at((remaining_players.len() as f32 / 2.0).ceil() as usize);

            let players_to_promote = players_to_promote.iter().filter(|player| {
                signups.iter().any(|signup| signup.discord_id == player.player.id)
            }).map(|player| player.clone()).collect::<Vec<_>>();

            let players_to_keep = players_to_keep.iter().filter(|player| {
                signups.iter().any(|signup| signup.discord_id == player.player.id)
            }).map(|player| player.clone()).collect::<Vec<_>>();

            let demoting = demoting.iter().filter(|player| {
                signups.iter().any(|signup| signup.discord_id == player.player.id)
            }).map(|player| player.clone()).collect::<Vec<_>>();
            
            players_to_demote = demoting.to_vec();

            if index > 0 {
                division_blueprints.get_mut(index - 1).unwrap().players.extend(players_to_promote.iter().map(|player| PlayerBlueprint{
                    tag: player.player.tag.clone(),
                    id: player.player.id.clone(),
                }));
            } 
            else {
                division_blueprint.players.extend(players_to_promote.iter().map(|player| PlayerBlueprint{
                    tag: player.player.tag.clone(),
                    id: player.player.id.clone(),
                }));
            }

            division_blueprint.players.extend(players_to_keep.iter().map(|player| PlayerBlueprint{
                tag: player.player.tag.clone(),
                id: player.player.id.clone(),
            }));

            if index == ordered_divisions.len() - 1 {
                division_blueprint.players.extend(players_to_demote.iter().map(|player| PlayerBlueprint{
                    tag: player.player.tag.clone(),
                    id: player.player.id.clone(),
                }));
            }

            division_blueprints.push(division_blueprint);
        }

        let involoved_players = division_blueprints.iter().flat_map(|division| division.players.clone()).collect::<Vec<_>>();

        let new_signups = signups.iter().filter(|signup| {
            !involoved_players.iter().any(|player| player.id == signup.discord_id)
        }).map(|signup| PlayerBlueprint{
            tag: signup.username.clone(),
            id: signup.discord_id.clone(),
        }).collect::<Vec<_>>();


        return PlanBlueprint {
            divisions: division_blueprints,
            players_to_sort: new_signups,
            end_timestamp: None,
            pause_end_timestamp: None,
            season: 0
        };
    }
}