use derive_getters::Getters;
use serde::{Deserialize, Serialize};

fn de_usize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<usize, D::Error> {
    Ok(match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::String(s) => s.parse().map_err(serde::de::Error::custom)?,
        serde_json::Value::Number(num) => num.as_u64().ok_or(serde::de::Error::custom("Invalid number"))? as usize,
        _ => return Err(serde::de::Error::custom("wrong type"))
    })
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone, Debug, Getters, PartialEq)]
#[serde(rename_all(deserialize = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Data of a single Clan
pub struct Clan {
    #[serde(rename(deserialize = "Clan", serialize = "name"))]
    #[cfg_attr(feature = "utoipa", schema(example = "Clan"))]
    /// Name of this clan
    name: String,

    #[cfg_attr(feature = "utoipa", schema(example = "CLN"))]
    /// Tag of this clan
    tag: String,

    #[serde(rename(deserialize = "XP"), deserialize_with = "de_usize")]
    #[cfg_attr(feature = "utoipa", schema(example = 1_000_000))]
    /// The total experience of this clan
    xp: usize,
    #[serde(deserialize_with = "de_usize")]
    #[cfg_attr(feature = "utoipa", schema(example = 100))]
    /// The maximum amount of players in this clan
    max_players: usize,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone, Debug, Getters, PartialEq)]
#[serde(rename_all(deserialize = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Data of a single player
pub struct Player {
    #[cfg_attr(feature = "utoipa", schema(example = "Player"))]
    /// The name of this player
    name: String,

    #[serde(deserialize_with = "de_usize")]
    #[cfg_attr(feature = "utoipa", schema(example = 100))]
    /// The value, this is different for each leaderboard. On the kills leaderboard, it's the amount of kills. On the XP leaderboard, it's the amount of XP.
    value: usize,
}

/// Leaderboard data
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone, Debug, Getters, PartialEq, Default)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Leaderboard {
    top_clans: Vec<Clan>,
    most_xp: Vec<Player>,
    most_heals: Vec<Player>,
    most_revives: Vec<Player>,
    most_vehicles_destroyed: Vec<Player>,
    most_vehicle_repairs: Vec<Player>,
    most_roadkills: Vec<Player>,
    longest_kills: Vec<Player>,
    most_objectives_complete: Vec<Player>,
    most_kills: Vec<Player>,
}

impl From<Vec<Leaderboards>> for Leaderboard {
    fn from(lbs: Vec<Leaderboards>) -> Self {
        let mut leaderboard = Leaderboard {
            top_clans: Vec::new(),
            most_xp: Vec::new(),
            most_kills: Vec::new(),
            most_heals: Vec::new(),
            most_revives: Vec::new(),
            most_vehicles_destroyed: Vec::new(),
            most_vehicle_repairs: Vec::new(),
            most_roadkills: Vec::new(),
            longest_kills: Vec::new(),
            most_objectives_complete: Vec::new(),
        };

        for lb in lbs {
            match lb {
                Leaderboards::TopClans(d) => leaderboard.top_clans = d,
                Leaderboards::MostXP(d) => leaderboard.most_xp = d,
                Leaderboards::MostKills(d) => leaderboard.most_kills = d,
                Leaderboards::MostHeals(d) => leaderboard.most_heals = d,
                Leaderboards::MostRevives(d) => leaderboard.most_revives = d,
                Leaderboards::MostVehiclesDestroyed(d) => leaderboard.most_vehicles_destroyed = d,
                Leaderboards::MostVehicleRepairs(d) => leaderboard.most_vehicle_repairs = d,
                Leaderboards::MostRoadkills(d) => leaderboard.most_roadkills = d,
                Leaderboards::MostLongestKill(d) => leaderboard.longest_kills = d,
                Leaderboards::MostObjectivesComplete(d) => leaderboard.most_objectives_complete = d,
            }
        }

        leaderboard
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum Leaderboards {
    TopClans(Vec<Clan>),
    MostXP(Vec<Player>),
    MostKills(Vec<Player>),
    MostHeals(Vec<Player>),
    MostRevives(Vec<Player>),
    MostVehiclesDestroyed(Vec<Player>),
    MostVehicleRepairs(Vec<Player>),
    MostRoadkills(Vec<Player>),
    MostLongestKill(Vec<Player>),
    MostObjectivesComplete(Vec<Player>),
}