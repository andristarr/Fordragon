use std::sync::{Arc, RwLock};

use crate::server::{
    opcode::OpCode, packet_handler::enter_packet_handler::EnterPacketHandler,
    state::authorization_handler::AuthorizationHandlerTrait,
};

use super::{move_packet_handler::MovePacketHandler, packet_handler::PacketHandler};

pub struct PacketHandlerBuilder {
    handler: PacketHandler,
}

impl Default for PacketHandlerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketHandlerBuilder {
    pub fn new() -> Self {
        PacketHandlerBuilder {
            handler: PacketHandler::new(),
        }
    }

    pub fn with_enter_handler(
        mut self,
        authorization_handler: Arc<RwLock<dyn AuthorizationHandlerTrait>>,
    ) -> Self {
        self.handler.handlers.insert(
            OpCode::Enter,
            Box::new(EnterPacketHandler::new(authorization_handler)),
        );
        self
    }

    pub fn with_move_handler(
        mut self,
        authorization_handler: Arc<RwLock<dyn AuthorizationHandlerTrait>>,
    ) -> Self {
        self.handler.handlers.insert(
            OpCode::Move,
            Box::new(MovePacketHandler::new(authorization_handler)),
        );
        self
    }

    pub fn build(self) -> PacketHandler {
        self.handler
    }
}
