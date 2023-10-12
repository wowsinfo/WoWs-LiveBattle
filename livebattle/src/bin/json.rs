use wows_live_battle::{reader::live_reader::parse_live_replay, setup_logger};
use wows_replays::analyzer::decoder::DecoderBuilder;

fn main() {
    setup_logger("info", "off");
    println!("WoWs LiveBattle JSON");

    let input: &str = r"C:\Games\World_of_Warships\replays\";
    let json_builder = DecoderBuilder::new(false, true, Some("live.json"));

    parse_live_replay(input, &json_builder, 50, 1000);
}
