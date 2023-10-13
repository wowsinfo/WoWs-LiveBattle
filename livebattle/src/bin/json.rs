use wows_live_battle::{parser::replay_parser::parse_replay, setup_logger};
use wows_replays::analyzer::decoder::DecoderBuilder;

fn main() {
    setup_logger("info", "off");
    println!("WoWs LiveBattle JSON");
    // input is the first argument
    let input = std::env::args().nth(1).expect("replay file path is not provided");
    let json_builder = DecoderBuilder::new(false, true, Some("live.json"));

    match parse_replay(&input, &json_builder) {
        Ok(_) => println!("ok"),
        Err(e) => println!("error: {}", e),
    }
}
