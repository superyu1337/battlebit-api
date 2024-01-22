#![doc = include_str!("../DOCS_README.md")]

mod enums;
mod endpoints;
mod error;

#[cfg(feature="async")]
mod api_async;

#[cfg(not(feature="async"))]
mod api;

#[cfg(feature="async")]
pub use api_async::BBApi;

#[cfg(not(feature="async"))]
pub use api::BBApi;

pub use error::Error;
pub use endpoints::{ServerData, Leaderboard, Player, Clan};
pub use enums::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_list() {
        let api = api::BBApi::default();
        if let Err(e) = api.server_list() {
            panic!("{e}");
        }
    }
}
