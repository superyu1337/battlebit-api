use derive_getters::Getters;
use serde::Deserialize;

use crate::enums::{MapSize, Gamemode, Region, DayNight, AntiCheat};

/// Data of a single server
#[allow(dead_code)]
#[derive(Deserialize, Debug, Getters)]
pub struct ServerData {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Map")]
    map: String,

    #[serde(rename = "MapSize")]
    map_size: MapSize,

    #[serde(rename = "Gamemode")]
    gamemode: Gamemode,

    #[serde(rename = "Region")]
    region: Region,

    #[serde(rename = "Players")]
    player_count: u8,

    #[serde(rename = "QueuePlayers")]
    queued_player_count: u8,

    #[serde(rename = "MaxPlayers")]
    max_players: u8,

    #[serde(rename = "Hz")]
    hz: u8,

    #[serde(rename = "DayNight")]
    day_night: DayNight,

    #[serde(rename = "IsOfficial")]
    is_official: bool,

    #[serde(rename = "HasPassword")]
    has_password: bool,

    #[serde(rename = "AntiCheat")]
    anti_cheat: AntiCheat,

    #[serde(rename = "Build")]
    build: String,
}