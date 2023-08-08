mod enums;
mod endpoints;
mod error;
mod api;

pub use api::BBApi;
pub use error::Error;
pub use endpoints::*;
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
