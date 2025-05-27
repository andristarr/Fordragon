use std::sync::{Arc, Mutex};

use crate::server::state::{packet_id_generator::PacketIdGenerator, ticker::TickerTrait};

use super::packet_sender::ServerPacketSender;

pub struct ServerPacketSenderBuilder;

impl ServerPacketSenderBuilder {
    pub fn build(
        ticker: Arc<Mutex<dyn TickerTrait>>,
        packet_id_generator: Arc<Mutex<PacketIdGenerator>>,
    ) -> ServerPacketSender {
        ServerPacketSender::new(ticker, packet_id_generator)
    }
}
