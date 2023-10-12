use wows_replays::analyzer::{Analyzer, AnalyzerBuilder};
use wows_replays::version::Version;
use wows_replays::ReplayMeta;

use super::server::WebsocketServer;

pub struct WebSocketServerBuilder {
    pub ip: String,
    pub port: u16,
}

impl WebSocketServerBuilder {
    pub fn new(ip: String, port: u16) -> Self {
        Self { ip, port }
    }
}

impl AnalyzerBuilder for WebSocketServerBuilder {
    fn build(&self, meta: &ReplayMeta) -> Box<dyn Analyzer> {
        let server_address = format!("{}:{}", self.ip, self.port);
        let mut server = WebsocketServer::new(
            server_address,
            Version::from_client_exe(&meta.clientVersionFromExe),
        );
        server.start();
        Box::new(server)
    }
}
