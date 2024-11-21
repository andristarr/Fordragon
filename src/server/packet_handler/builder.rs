use std::sync::{Arc, Mutex};

use crate::server::state::{
    state_handler::{ServerStateHandler, StateHandler},
    ticker::TickerTrait,
};

use super::packet_handler::ServerPacketHandler;

pub struct ServerPacketHandlerBuilder;

impl ServerPacketHandlerBuilder {
    pub fn build(
        state_handler: ServerStateHandler,
        ticker: Arc<Mutex<dyn TickerTrait>>,
    ) -> ServerPacketHandler {
        ServerPacketHandler::new(state_handler, ticker)
    }
}
