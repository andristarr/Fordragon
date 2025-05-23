use std::sync::{Arc, Mutex};

use crate::server::state::{
    state_handler::{ServerStateHandler, StateHandler},
    ticker::TickerTrait,
};

use super::packet_sender::ServerPacketSender;

pub struct ServerPacketSenderBuilder;

impl ServerPacketSenderBuilder {
    pub fn build(ticker: Arc<Mutex<dyn TickerTrait>>) -> ServerPacketSender {
        ServerPacketSender::new(ticker)
    }
}
