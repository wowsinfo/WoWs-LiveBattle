use std::{collections::HashMap, net::TcpListener, thread::spawn, sync::{Arc, Mutex}};

use tungstenite::{accept, WebSocket};
use wows_replays::{
    analyzer::{
        decoder::{DecodedPacket, DecodedPacketPayload},
        Analyzer,
    },
    packet2::Packet,
    version::Version,
};

pub struct WebsocketServer {
    pub address: String,
    pub version: Version,
    server: Option<TcpListener>,
    connected_clients: HashMap<usize, WebSocket<std::net::TcpStream>>,
    next_client_id: usize,
}

impl WebsocketServer {
    pub fn new(address: String, version: Version) -> Self {
        Self {
            address,
            version,
            server: None,
            connected_clients: HashMap::new(),
            next_client_id: 0,
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.server = TcpListener::bind(self.address.clone()).ok();
        if let Some(server) = &self.server {
            while let Some(stream) = server.incoming().next() {
                let stream = stream?;
                let websocket = accept(stream)?;
                self.connected_clients.insert(self.next_client_id, websocket);
                self.next_client_id += 1;
            }
        }
        Ok(())
    }

    fn stop(&self) {
        // stop the server
        if let Some(server) = &self.server {
            drop(server);
        }
    }
}

impl Analyzer for WebsocketServer {
    fn process(&mut self, packet: &Packet<'_, '_>) {
        let decoded = DecodedPacket::from(&self.version, false, packet);

        match decoded.payload {
            DecodedPacketPayload::Unknown(_) => {
                return; // ignore unknown packets
            }
            _ => {
                let encoded = serde_json::to_string(&decoded).unwrap();
                let mut closed_sockets = Vec::new();
                for (id, client) in self.connected_clients.iter_mut() {
                    if let Err(_) =
                        client.write(tungstenite::Message::Text(encoded.to_string()))
                    {
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
        }
    }

    fn finish(&self) {
        // stop the server
        self.stop()
    }
}
