use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::enums::{MapSize, Gamemode, Region, DayNight, AntiCheat};

/// Data of a single server
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone, Debug, Getters, PartialEq)]
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
    /// Get field `player_count` from instance of `ServerData`.  
    /// SAFETY: Unless Battlebit upgrades their engine, this number should fit into a u8.
    player_count: u8,

    #[serde(rename = "QueuePlayers")]
    /// Get field `queued_player_count` from instance of `ServerData`.  
    /// SAFETY: The queued player count should realistically never reach 65_535.
    queued_player_count: u16,

    #[serde(rename = "MaxPlayers")]
    /// Get field `max_players` from instance of `ServerData`.  
    /// SAFETY: Unless Battlebit upgrades their engine, this number should fit into a u8.
    max_players: u8,

    #[serde(rename = "Hz")]
    /// Get field `hz` from instance of `ServerData`.  
    /// SAFETY: Unless Battlebit upgrades their engine, this number should fit into a u8.
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

impl ServerData {
    /// Small check if this ServerData has `Unknown` fields.  
    /// Mostly used by me to check if this API client is outdated.
    pub fn has_unknown(&self) -> bool {
        if *self.anti_cheat() == AntiCheat::Unknown { return true }
        if *self.region() == Region::Unknown { return true }
        if *self.gamemode() == Gamemode::Unknown { return true }
        if *self.map_size() == MapSize::Unknown { return true }

        false
    }
}