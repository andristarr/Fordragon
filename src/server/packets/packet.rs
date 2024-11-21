use serde::{Deserialize, Serialize};

use crate::server::opcode::OpCode;

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub id: u128,
    pub opcode: OpCode,
    pub data: String,
}
