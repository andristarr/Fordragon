use serde::{Deserialize, Serialize};

use crate::server::opcode::OpCode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Packet {
    pub id: u128,
    pub opcode: OpCode,
    pub data: String,
}

impl Packet {
    pub fn new(id: u128, opcode: OpCode, data: String) -> Self {
        Packet {
            id,
            opcode,
            data,
        }
    }
}
