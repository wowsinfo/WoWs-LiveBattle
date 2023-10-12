use wows_live_battle::{
    reader::live_reader::parse_live_replay, server::websocket_server::WebSocketServerBuilder,
    setup_logger,
};

fn main() {
    setup_logger("info", "off");
    println!("WoWs LiveBattle Server");

    let input = r"C:\Games\World_of_Warships\replays\";
    let websocket_builder = WebSocketServerBuilder::new("127.0.0.1".to_string(), 8615);

    parse_live_replay(input, &websocket_builder, 50, 1000);
}
