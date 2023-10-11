use serde::{Deserialize, Serialize};
use crate::server::opcode::OpCode;

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    opcode: OpCode,
    data: Vec<u8>
}