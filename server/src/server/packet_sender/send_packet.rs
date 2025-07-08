use serde::{Deserialize, Serialize};

use crate::server::{opcode::OpCode, packet_sender::TargetAddress};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendPacket {
    pub packet_data: String,
    pub opcode: OpCode,
    pub addr: TargetAddress,
}

impl SendPacket {
    pub fn new(packet_data: String, opcode: OpCode, addr: TargetAddress) -> Self {
        SendPacket {
            packet_data,
            opcode,
            addr,
        }
    }
}
