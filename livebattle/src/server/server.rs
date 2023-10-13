use log::debug;
use simple_websockets::{Event, Responder};
use std::{
    collections::HashMap,
    net::TcpListener,
    sync::mpsc,
    thread::{sleep, spawn},
    time::Duration,
};
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
    packet_sender: Option<mpsc::Sender<String>>,
}

impl WebsocketServer {
    pub fn new(address: String, version: Version) -> Self {
        Self {
            address,
            version,
            packet_sender: None,
        }
    }

    pub fn start(&mut self) {
        let listener = TcpListener::bind(&self.address).expect("The address is not available");
        let event_hub = simple_websockets::launch_from_listener(listener)
            .expect("The websocket server could not be started");

        let (client_sender, client_receiver) = mpsc::channel();
        let (packet_sender, packet_receiver) = mpsc::channel();
        self.packet_sender = Some(packet_sender);

        // run the server in a separate thread
        spawn(move || loop {
            match event_hub.poll_event() {
                Event::Connect(client_id, responder) => {
                    debug!("A new client is connected with id {}", client_id);
                    client_sender.send((client_id, Some(responder))).unwrap();
                }
                Event::Disconnect(client_id) => {
                    debug!("Client {} disconnected...", client_id);
                    client_sender.send((client_id, None)).unwrap();
                }
                Event::Message(client_id, message) => {
                    debug!(
                        "Received a message from client #{}: {:?}",
                        client_id, message
                    );
                }
            }
        });

        // receive messages from another thread
        spawn(move || {
            let mut clients: HashMap<u64, Responder> = HashMap::new();
            loop {
                match client_receiver.recv_timeout(Duration::from_millis(1)).ok() {
                    Some((client_id, responder)) => match responder {
                        Some(responder) => {
                            debug!("A new client is connected with id {}", client_id);
                            clients.insert(client_id, responder);
                        }
                        None => {
                            debug!("Client {} disconnected...", client_id);
                            clients.remove(&client_id);
                        }
                    },
                    None => {}
                }

                match packet_receiver.recv_timeout(Duration::from_millis(1)).ok() {
                    Some(packet) => {
                        for (_, client) in clients.iter() {
                            client.send(simple_websockets::Message::Text(packet.clone()));
                        }
                    }
                    None => {}
                }

                sleep(Duration::from_millis(2));
            }
        });
    }
}

impl Analyzer for WebsocketServer {
    fn process(&mut self, packet: &Packet<'_, '_>) {
        if let Some(sender) = &self.packet_sender {
            let decoded = DecodedPacket::from(&self.version, false, packet);

            match decoded.payload {
                DecodedPacketPayload::Unknown(_) => {
                    return; // ignore unknown packets
                }
                DecodedPacketPayload::Position(_) => {
                    return; // ignore position packets
                }
                _ => {
                    let encoded = serde_json::to_string(&decoded).unwrap();
                    sender.send(encoded).unwrap();
                }
            }
        }
    }

    fn finish(&self) {}
}
