use derive_more::Display;
use serde::Deserialize;

#[derive(Deserialize, Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum AntiCheat {
    #[serde(rename = "EAC")]
    EasyAntiCheat,
    #[serde(other)]
    Unknown
}

#[derive(Deserialize, Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum DayNight {
    Day,
    Night,
}

#[derive(Deserialize, Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum Gamemode {
    #[serde(rename = "CONQ")]
    Conquest,
    #[serde(rename = "INFCONQ")]
    InfanteryConquest,
    #[serde(rename = "RUSH")]
    Rush,
    #[serde(rename = "FRONTLINE")]
    Frontline,
    #[serde(rename = "DOMI")]
    Domination,
    #[serde(rename = "ELI")]
    Elimination,
    #[serde(rename = "TDM")]
    TeamDeathmatch,
    CaptureTheFlag,
    VoxelFortify,
    #[serde(rename = "FFA")]
    FreeForAll,
    #[serde(rename = "19")]
    /// Formely unknown gamemode, no idea what this exactly is atm.
    Gamemode19,
    #[serde(other)]
    Unknown
}

#[derive(Deserialize, Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum MapSize {
    Ultra,
    Big,
    Medium,
    Small,
    Tiny,
    #[serde(other)]
    Unknown
}

#[derive(Deserialize, Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum Region {
    #[serde(rename = "Europe_Central")]
    Europe,
    #[serde(rename = "Japan_Central")]
    Japan,
    #[serde(rename = "Australia_Central")]
    Australia,
    #[serde(rename = "America_Central")]
    America,
    #[serde(rename = "Brazil_Central")]
    Brazil,
    #[serde(rename = "Asia_Central")]
    Asia,
    #[serde(rename = "Developer_Server")]
    DeveloperServer,
    #[serde(other)]
    Unknown
}