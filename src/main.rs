fn main() {
    let api = battlebit_api::BBApi::default();
    let server_list = api.server_list().unwrap();

    println!("{server_list:#?}")
}