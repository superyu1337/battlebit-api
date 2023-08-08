use derive_more::Display;
use serde::Deserialize;

#[derive(Deserialize, Debug, Display)]
pub enum AntiCheat {
    #[serde(rename = "EAC")]
    EasyAntiCheat,
    #[serde(other)]
    Unknown
}

#[derive(Deserialize, Debug, Display)]
pub enum DayNight {
    Day,
    Night,
}

#[derive(Deserialize, Debug, Display)]
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
    #[serde(other)]
    Unknown
}

#[derive(Deserialize, Debug, Display)]
pub enum MapSize {
    Ultra,
    Big,
    Medium,
    Small,
    #[serde(other)]
    Unknown
}

#[derive(Deserialize, Debug, Display)]
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

    #[serde(rename = "Developer_Server")]
    DeveloperServer,

    #[serde(other)]
    Unknown
}