use crate::server::state::state_handler::StateHandler;

use super::packet_handler::ServerPacketHandler;

pub struct ServerPacketHandlerBuilder;

impl ServerPacketHandlerBuilder {
    pub fn build<T: StateHandler>(state_handler: T) -> ServerPacketHandler<T> {
        ServerPacketHandler::new(state_handler)
    }
}
