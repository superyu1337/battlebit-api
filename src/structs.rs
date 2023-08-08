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

    #[serde(rename = "TDM")]
    TeamDeathmatch,

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
    EuropeCentral,

    #[serde(rename = "Japan_Central")]
    JapanCentral,

    #[serde(rename = "Australia_Central")]
    AustraliaCentral,

    #[serde(rename = "America_Central")]
    AmericaCentral,

    #[serde(rename = "Brazil_Central")]
    BrazilCentral,

    #[serde(rename = "Developer_Server")]
    DeveloperServer,
}