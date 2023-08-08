pub mod structs;
mod endpoints;
mod error;
mod api;

pub use api::BBApi;
pub use error::Error;
pub use endpoints::*;

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
