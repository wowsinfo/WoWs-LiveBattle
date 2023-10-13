use wows_live_battle::{reader::live_reader::start_live_battle_server, setup_logger};

fn main() {
    setup_logger("debug", "info");
    println!("WoWs LiveBattle Server");

    let input = r"C:\Games\World_of_Warships\replays\";
    start_live_battle_server("127.0.0.1", input, 10, 2000);
}
