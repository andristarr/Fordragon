use std::sync::{Arc, Mutex};

use crate::server::state::{state_handler::StateHandler, ticker::TickerTrait};

use super::packet_receiver::ServerPacketReceiver;

pub struct ServerPacketReceiverBuilder;

impl ServerPacketReceiverBuilder {
    pub fn build(
        state_handler: Box<dyn StateHandler>,
        ticker: Arc<Mutex<dyn TickerTrait>>,
    ) -> ServerPacketReceiver {
        ServerPacketReceiver::new(state_handler, ticker)
    }
}
