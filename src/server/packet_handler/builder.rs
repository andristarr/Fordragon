use crate::server::opcode::OpCode;

use super::{
    move_packet_handler::MovePacketHandler, packet_handler::PacketHandler,
    spawn_packet_handler::SpawnPacketHandler,
};

pub struct PacketHandlerBuilder {
    handler: PacketHandler,
}

impl PacketHandlerBuilder {
    pub fn new() -> Self {
        PacketHandlerBuilder {
            handler: PacketHandler::new(),
        }
    }

    pub fn with_spawn_handler(mut self) -> Self {
        self.handler
            .handlers
            .insert(OpCode::Spawn, Box::new(SpawnPacketHandler::new()));
        self
    }

    pub fn with_move_handler(mut self) -> Self {
        self.handler
            .handlers
            .insert(OpCode::Movement, Box::new(MovePacketHandler::new()));
        self
    }

    pub fn build(self) -> PacketHandler {
        self.handler
    }
}
