use std::sync::{Arc, Mutex};

use crate::server::state::{
    state_handler::{ServerStateHandler, StateHandler},
    ticker::TickerTrait,
};

use super::packet_receiver::ServerPacketReceiver;

pub struct ServerPacketReceiverBuilder;

impl ServerPacketReceiverBuilder {
    pub fn build(
        state_handler: ServerStateHandler,
        ticker: Arc<Mutex<dyn TickerTrait>>,
    ) -> ServerPacketReceiver {
        ServerPacketReceiver::new(state_handler, ticker)
    }
}
