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
    /// The name of this server.
    name: String,
    #[cfg_attr(feature = "utoipa", schema(example = "GenericMap"))]
    /// The map name that the server runs.
    map: String,
    #[cfg_attr(feature = "utoipa", schema(example = MapSize::Ultra))]
    /// The size of the map.
    map_size: MapSize,
    #[cfg_attr(feature = "utoipa", schema(example = Gamemode::Conquest))]
    /// The current gamemode on this server.
    gamemode: Gamemode,
    #[cfg_attr(feature = "utoipa", schema(example = Region::Europe))]
    /// The region where this server is in.
    region: Region,

    #[serde(rename(deserialize = "Players"))]
    #[cfg_attr(feature = "utoipa", schema(example = 124))]
    /// The amount of players currently playing on this server.
    // SAFETY: Unless Battlebit upgrades their engine, this number should fit into a u8.
    player_count: u8,

    #[serde(rename(deserialize = "QueuePlayers"))]
    #[cfg_attr(feature = "utoipa", schema(example = 2))]
    /// The amount of players currently in queue to join this server.
    // SAFETY: The queued player count should realistically never reach 65_535.
    queued_player_count: u16,

    #[cfg_attr(feature = "utoipa", schema(example = 254))]
    /// The maximum amount of players that can be on this server.
    // SAFETY: Unless Battlebit upgrades their engine, this number should fit into a u8.
    max_players: u8,

    #[cfg_attr(feature = "utoipa", schema(example = 120))]
    /// The refreshrate/tickrate of this server.
    // SAFETY: Unless Battlebit upgrades their engine, this number should fit into a u8.
    hz: u8,

    #[cfg_attr(feature = "utoipa", schema(example = DayNight::Day))]
    // Time of day on the map, see DayNight for more info.
    day_night: DayNight,
    #[cfg_attr(feature = "utoipa", schema(example = false))]
    /// Whether or not this server is official.
    is_official: bool,
    #[cfg_attr(feature = "utoipa", schema(example = false))]
    /// Whether or not this server has a password.
    has_password: bool,
    #[cfg_attr(feature = "utoipa", schema(example = AntiCheat::EasyAntiCheat))]
    /// The type of anticheat used on this server. See AntiCHeat for more info.
    anti_cheat: AntiCheat,
    #[cfg_attr(feature = "utoipa", schema(example = "Production 2.2.5 Hotfix"))]
    /// The build that this server runs.
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