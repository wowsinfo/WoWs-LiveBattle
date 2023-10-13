use std::{fs::File, sync::mpsc::Sender};

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
    output: Option<Box<dyn std::io::Write>>,
    packets: Vec<String>,
}

impl PacketSender {
    pub fn new(
        version: Version,
        packet_sender: Sender<String>,
        output_file: Option<&str>,
    ) -> Self {
        if let Some(output_file) = output_file {
            let output = File::create(output_file).unwrap();
            Self {
                version,
                packet_sender,
                output: Some(Box::new(output)),
                packets: Vec::new(),
            }
        } else {
            Self {
                version,
                packet_sender,
                output: None,
                packets: Vec::new(),
            }
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
                if let Some(output) = self.output.as_mut() {
                    writeln!(output, "{}", encoded.clone()).unwrap();
                }

                if self.packets.len() > 10 {
                    // send on a bundle
                    let packet_array = self.packets.join(",");
                    let packet_array_json = format!("[{}]", packet_array);
                    self.packet_sender.send(packet_array_json).unwrap();
                    self.packets.clear();
                } else {
                    self.packets.push(encoded);
                }
            }
        }
    }

    fn finish(&self) {}
}
