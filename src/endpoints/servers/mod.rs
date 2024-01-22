use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::enums::{MapSize, Gamemode, Region, DayNight, AntiCheat};

/// Data of a single server
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone, Debug, Getters, PartialEq)]
#[serde(rename_all(deserialize = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ServerData {
    #[cfg_attr(feature = "utoipa", schema(example = "Super Awesome Server"))]
    name: String,
    #[cfg_attr(feature = "utoipa", schema(example = "GenericMap"))]
    map: String,
    #[cfg_attr(feature = "utoipa", schema(example = MapSize::Ultra))]
    map_size: MapSize,
    #[cfg_attr(feature = "utoipa", schema(example = Gamemode::Conquest))]
    gamemode: Gamemode,
    #[cfg_attr(feature = "utoipa", schema(example = Region::Europe))]
    region: Region,

    #[serde(rename(deserialize = "Players"))]
    #[cfg_attr(feature = "utoipa", schema(example = 124))]
    /// Get field `player_count` from instance of `ServerData`.  
    /// SAFETY: Unless Battlebit upgrades their engine, this number should fit into a u8.
    player_count: u8,

    #[serde(rename(deserialize = "QueuePlayers"))]
    #[cfg_attr(feature = "utoipa", schema(example = 2))]
    /// Get field `queued_player_count` from instance of `ServerData`.  
    /// SAFETY: The queued player count should realistically never reach 65_535.
    queued_player_count: u16,

    #[cfg_attr(feature = "utoipa", schema(example = 254))]
    /// Get field `max_players` from instance of `ServerData`.  
    /// SAFETY: Unless Battlebit upgrades their engine, this number should fit into a u8.
    max_players: u8,

    #[cfg_attr(feature = "utoipa", schema(example = 120))]
    /// Get field `hz` from instance of `ServerData`.  
    /// SAFETY: Unless Battlebit upgrades their engine, this number should fit into a u8.
    hz: u8,

    #[cfg_attr(feature = "utoipa", schema(example = DayNight::Day))]
    day_night: DayNight,
    #[cfg_attr(feature = "utoipa", schema(example = false))]
    is_official: bool,
    #[cfg_attr(feature = "utoipa", schema(example = false))]
    has_password: bool,
    #[cfg_attr(feature = "utoipa", schema(example = AntiCheat::EasyAntiCheat))]
    anti_cheat: AntiCheat,
    #[cfg_attr(feature = "utoipa", schema(example = "Production 2.2.5 Hotfix"))]
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