use battlebit_api::BBApi;

fn main() {
    let api = BBApi::new();
    let server_list = api.server_list()
        .expect("Retrieving server list");

    server_list.iter().for_each(|server| {

        if *server.gamemode() == battlebit_api::Gamemode::Conquest {
            println!("{} [{}, {}, {}] ({}, {}, {})", 
                server.name(), 
                server.gamemode(), 
                server.map(),
                server.map_size(),
                server.hz(), 
                server.anti_cheat(),
                server.build()
            )
        }
    });
}