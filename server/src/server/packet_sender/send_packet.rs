use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::server::opcode::OpCode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendPacket {
    pub packet_data: String,
    pub opcode: OpCode,
    pub addr: Option<SocketAddr>,
}

impl SendPacket {
    pub fn new(packet_data: String, opcode: OpCode, addr: Option<SocketAddr>) -> Self {
        SendPacket {
            packet_data,
            opcode,
            addr,
        }
    }
}
