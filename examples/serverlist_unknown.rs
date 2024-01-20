use battlebit_api::BBApi;

fn main() {
    let api = BBApi::new();

    let server_list = api.server_list()
        .expect("Retrieving server list");

    let servers_with_unknown: Vec<battlebit_api::ServerData> = server_list
        .into_iter()
        .filter(|server| server.has_unknown())
        .collect();

    if servers_with_unknown.is_empty() {
        println!("No servers with unknown fields found!")
    } else {
        servers_with_unknown.iter().for_each(|server| {
            println!("{} [{}, {}, {}] ({}, {}, {})", 
                server.name(), 
                server.gamemode(), 
                server.map(),
                server.map_size(),
                server.hz(), 
                server.anti_cheat(),
                server.build()
            )
        });
    }
}