use battlebit_api::{BBApi, Clan, Player};

fn print_player(leaderboard: &[Player], t: &'static str, u: &'static str) {
    let p = leaderboard
        .first()
        .expect("Leaderboard is empty");

    println!(
        "{t}: {} ({} {u})",
        p.name(),
        p.value()
    )
}

fn print_clan(leaderboard: &[Clan]) {
    let c = leaderboard
        .first()
        .expect("Leaderboard is empty");

    println!(
        "Top clan: {} [{}] ({} XP)",
        c.name(),
        c.tag(),
        c.xp()
    )
}

fn main() {
    let api = BBApi::new();
    let lb = api.leaderboard()
        .expect("Retrieving leaderboard");

    print_clan(lb.top_clans());

    print_player(lb.most_kills(), "Most kills", "Kills");
    print_player(lb.most_roadkills(), "Most roadkills", "Roadkills");
    print_player(lb.longest_kills(), "Longest kill", "Meters");

    print_player(lb.most_xp(), "Most XP", "XP");
    print_player(lb.most_heals(), "Most heal", "HP");
    print_player(lb.most_revives(), "Most revives", "Revives");

    print_player(lb.most_objectives_complete(), "Most objectives", "Objectives");

    print_player(lb.most_vehicles_destroyed(), "Vehicles destroyed", "Vehicles");
    print_player(lb.most_vehicle_repairs(), "Vehicle repairs", "Repairs");
}