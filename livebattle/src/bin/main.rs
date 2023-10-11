use std::path::PathBuf;

use wows_live_battle::parser::replay_parser::parse_replay;
use wows_replays::analyzer::decoder::DecoderBuilder;

fn main() {
    println!("Hello, world!");

    let input = r"C:\Games\World_of_Warships\replays\20231007_212140_PBSC110-Minotaur_38_Canada.wowsreplay";
    let input = &PathBuf::from(input);
    let dump: DecoderBuilder = DecoderBuilder::new(
        false,
        true,
        Some("a.json"),
    );

    let result = parse_replay(input, dump);
    match result {
        Ok(_) => println!("ok"),
        Err(e) => println!("error: {:?}", e),
    }
}
