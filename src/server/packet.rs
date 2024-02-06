use crate::server::opcode::OpCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    opcode: OpCode,
    data: Vec<u8>,
}
