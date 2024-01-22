use reqwest::Url;

use reqwest::get;

use crate::endpoints;
use crate::error::Error;

fn fix_bom(data: &[u8]) -> &[u8] {
    if data.starts_with(&[0xEF, 0xBB, 0xBF]) {
        &data[3..]
    } else {
        data
    }
}

/// BattleBit API Struct
#[derive(Clone, PartialEq, Debug)]
pub struct BBApi(Url);

impl BBApi {

    /// Shorthand for `BBApi::Default()`  
    /// Creates a new BBApi with the default URL
    pub fn new() -> BBApi {
        BBApi::default()
    }

    /// Creates a new BBApi with the given URL
    pub fn with_url(url: Url) -> BBApi {
        BBApi(url)
    }

    /// Fetches the server list and puts it into a `Vec<ServerData>`
    pub async fn server_list(&self) -> Result<Vec<endpoints::ServerData>, Error> {
        let url = self.0.join("Servers/GetServerList")?;
        let data = get(url).await?.bytes().await?;
        let data = serde_json::from_slice(fix_bom(&data))?;

        Ok(data)
    }

    /// Fetches the leaderboard.
    pub async fn leaderboard(&self) -> Result<endpoints::Leaderboard, Error> {
        let url = self.0.join("Leaderboard/Get")?;
        let data = get(url).await?.bytes().await?;
        let result: Vec<endpoints::Leaderboards> = serde_json::from_slice(fix_bom(&data))?;
        Ok(endpoints::Leaderboard::from(result))
    }
}

impl Default for BBApi {
    /// Creates a new BBApi with the default URL  
    /// Default URL is `https://publicapi.battlebit.cloud/`
    fn default() -> Self {
        Self(Url::parse("https://publicapi.battlebit.cloud/").unwrap())
    }
}