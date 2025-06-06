use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::server::packets::packet::Packet;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceivedPacket {
    pub packet: Packet,
    pub addr: SocketAddr
}

impl ReceivedPacket {
    pub fn new(packet: Packet, addr: SocketAddr) -> Self {
        ReceivedPacket {
            packet,
            addr,
        }
    }
}
