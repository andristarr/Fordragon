use crate::server::state_handler::state_handler::StateHandler;

use super::packet_handler::{PacketHandler, ServerPacketHandler};

pub trait PacketHandlerBuilder {
    fn build(&self, state_handler: Box<dyn StateHandler>) -> impl PacketHandler;
}

pub struct ServerPacketHandlerBuilder;

impl PacketHandlerBuilder for ServerPacketHandlerBuilder {
    fn build(&self, state_handler: Box<dyn StateHandler>) -> impl PacketHandler {
        ServerPacketHandler { state_handler }
    }
}
