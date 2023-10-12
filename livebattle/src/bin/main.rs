use std::path::PathBuf;

use env_logger::Env;
use log::{debug, info};
use wows_live_battle::reader::live_reader::parse_live_replay;
use wows_replays::analyzer::decoder::DecoderBuilder;

fn setup_logger(debug: &str, release: &str) {
    if cfg!(debug_assertions) {
        env_logger::Builder::from_env(Env::default().default_filter_or(debug)).init();
    } else {
        env_logger::Builder::from_env(Env::default().default_filter_or(release)).init();
    }
}

fn main() {
    setup_logger("info", "off");
    println!("WoWs LiveBattle");

    // let input = r"C:\Games\World_of_Warships\replays\20231007_212140_PBSC110-Minotaur_38_Canada.wowsreplay";
    let input = r"C:\Games\World_of_Warships\replays\";
    let input = &PathBuf::from(input);
    let dump: DecoderBuilder = DecoderBuilder::new(false, true, Some("a.json"));

    parse_live_replay(
        input, 
        &dump, 
        50, 
        1000
    );
}
