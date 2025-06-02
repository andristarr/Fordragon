#![warn(unused_extern_crates)]

use std::sync::{Arc, Mutex};

use common::config::Config;
use server::{
    packet_receiver::packet_receiver::ServerPacketReceiver,
    packet_sender::builder::ServerPacketSenderBuilder,
    server::Server,
    state::{
        packet_id_generator::PacketIdGenerator, state_handler::ServerStateHandler, ticker::Ticker,
    },
};

mod common;
mod server;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .filter_level(log::LevelFilter::Info)
        .init();

    let config = Config::get().unwrap();

    let ticker = Ticker::new(config.tick_count);

    let ticker = Arc::new(Mutex::new(ticker));

    let packet_id_generator = PacketIdGenerator::new();

    let packet_id_generator = Arc::new(Mutex::new(packet_id_generator));

    let packet_sender =
        ServerPacketSenderBuilder::build(ticker.clone(), packet_id_generator.clone());

    let packet_sender = Arc::new(Mutex::new(packet_sender));

    let state_handler = ServerStateHandler::new(ticker.clone(), packet_sender.clone());

    let packet_receiver = ServerPacketReceiver::new(Box::new(state_handler), ticker.clone());

    let mut server = Server::new(Box::new(packet_receiver), packet_sender);

    let _ = server.run().await;
}
