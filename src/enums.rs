use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum AntiCheat {
    #[serde(rename = "EAC")]
    EasyAntiCheat
}

#[derive(Deserialize, Debug)]
pub enum DayNight {
    Day,
    Night
}

#[derive(Deserialize, Debug)]
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

    #[serde(rename = "ELIM")]
    Elimination,

    #[serde(rename = "TDM")]
    TeamDeathmatch,

    #[serde(rename = "CTF")]
    CaptureTheFlag,

    VoxelFortify
}

#[derive(Deserialize, Debug)]
pub enum MapSize {
    Ultra,
    Big,
    Medium,
    Small,
}

#[derive(Deserialize, Debug)]
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
}