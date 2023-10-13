use std::sync::mpsc::Sender;

use wows_replays::{
    analyzer::{
        decoder::{DecodedPacket, DecodedPacketPayload},
        Analyzer,
    },
    packet2::Packet,
    version::Version,
};

pub struct PacketSender {
    version: Version,
    packet_sender: Sender<String>,
}

impl PacketSender {
    pub fn new(version: Version, packet_sender: Sender<String>) -> Self {
        Self {
            version,
            packet_sender,
        }
    }
}

impl Analyzer for PacketSender {
    fn process(&mut self, packet: &Packet<'_, '_>) {
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
                self.packet_sender.send(encoded).unwrap();
            }
        }
    }

    fn finish(&self) {}
}
