use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum AntiCheat {
    #[serde(rename(deserialize = "EAC"))]
    EasyAntiCheat,
    #[serde(other)]
    Unknown
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum DayNight {
    Day,
    Night,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Gamemode {
    #[serde(rename(deserialize = "CONQ"))]
    Conquest,
    #[serde(rename(deserialize = "INFCONQ"))]
    InfanteryConquest,
    #[serde(rename(deserialize = "RUSH"))]
    Rush,
    #[serde(rename(deserialize = "FRONTLINE"))]
    Frontline,
    #[serde(rename(deserialize = "DOMI"))]
    Domination,
    #[serde(rename(deserialize = "ELI"))]
    Elimination,
    #[serde(rename(deserialize = "TDM"))]
    TeamDeathmatch,
    CaptureTheFlag,
    VoxelFortify,
    VoxelTrench,
    #[serde(rename(deserialize = "FFA"))]
    FreeForAll,
    #[serde(rename(deserialize = "19"))]
    /// Formely unknown gamemode, no idea what this exactly is atm.
    /// Maybe this is a custom gamemode?
    Gamemode19,
    #[serde(other)]
    Unknown
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum MapSize {
    Ultra,
    Big,
    Medium,
    Small,
    Tiny,
    #[serde(other)]
    Unknown
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Region {
    #[serde(rename(deserialize = "Europe_Central"))]
    Europe,
    #[serde(rename(deserialize = "Japan_Central"))]
    Japan,
    #[serde(rename(deserialize = "Australia_Central"))]
    Australia,
    #[serde(rename(deserialize = "America_Central"))]
    America,
    #[serde(rename(deserialize = "Brazil_Central"))]
    Brazil,
    #[serde(rename(deserialize = "Asia_Central"))]
    Asia,
    #[serde(rename(deserialize = "Developer_Server"))]
    DeveloperServer,
    #[serde(other)]
    Unknown
}