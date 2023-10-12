use std::{collections::HashMap, net::TcpListener, thread::spawn};

use tungstenite::{accept, WebSocket};
use wows_replays::analyzer::Analyzer;

pub struct WebsocketServer {
    pub address: String,
    server: Option<TcpListener>,
    connected_clients: HashMap<usize, WebSocket<std::net::TcpStream>>,
    next_client_id: usize,
}

impl WebsocketServer {
    pub fn new(address: String) -> Self {
        Self {
            address,
            server: None,
            connected_clients: HashMap::new(),
            next_client_id: 0,
        }
    }

    pub fn start(&mut self) {
        self.server = TcpListener::bind(self.address.clone()).ok();
        if let Some(server) = &self.server {
            for stream in server.incoming() {
                let websocket = accept(stream.unwrap()).unwrap();
                self.connected_clients
                    .insert(self.next_client_id, websocket);
                self.next_client_id += 1;
            }
        }
    }

    fn stop(&self) {
        // stop the server
        if let Some(server) = &self.server {
            drop(server);
        }
    }
}

impl Analyzer for WebsocketServer {
    fn process(&mut self, packet: &wows_replays::packet2::Packet<'_, '_>) {
        // send this packet to any connected clients
        let packet_string = serde_json::to_string(&packet).unwrap();
        let mut closed_sockets = Vec::new();
        for (id, client) in self.connected_clients.iter_mut() {
            if let Err(_) = client.write(tungstenite::Message::Text(packet_string.to_string())) {
                closed_sockets.push(*id);
            }
        }

        if closed_sockets.is_empty() {
            return;
        }

        for id in closed_sockets.iter() {
            self.connected_clients.remove(id);
        }
    }

    fn finish(&self) {
        // stop the server
        self.stop()
    }
}
