use log::debug;
use simple_websockets::{Event, Responder};
use std::{
    collections::HashMap,
    net::TcpListener,
    sync::mpsc,
    thread::{sleep, spawn},
    time::Duration,
};

pub struct WebsocketServer {
    pub address: String,
}

impl WebsocketServer {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn start(&self) -> mpsc::Sender<String> {
        let (client_sender, client_receiver) = mpsc::channel();
        let (packet_sender, packet_receiver) = mpsc::channel::<String>();

        let address = self.address.clone();
        spawn(move || {
            let tcp_listener = TcpListener::bind(address).expect("The address is not available");
            let events = simple_websockets::launch_from_listener(tcp_listener)
                .expect("The websocket server could not be started");
            loop {
                match events.poll_event() {
                    Event::Connect(client_id, responder) => {
                        debug!("A new client is connected with id {}", client_id);
                        let welcome = format!("{{\"type\": \"welcome\", \"id\": {}}}", client_id);
                        responder.send(simple_websockets::Message::Text(welcome));
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

        return packet_sender;
    }
}
