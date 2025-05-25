#![warn(unused_extern_crates)]

use std::sync::{Arc, Mutex};

use server::{
    packet_receiver::builder::ServerPacketReceiverBuilder,
    packet_sender::builder::ServerPacketSenderBuilder,
    server::Server,
    state::{state_handler::ServerStateHandler, ticker::Ticker},
};

mod common;
mod server;

#[tokio::main]
async fn main() {
    let mut log_builder = colog::basic_builder();

    log_builder
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_millis()
        .format_module_path(false)
        .format_target(false)
        .format_indent(Some(2));

    log_builder.init();

    let ticker = Ticker::new(8);

    let ticker = Arc::new(Mutex::new(ticker));

    let state_handler = ServerStateHandler::new(ticker.clone());

    let packet_receiver = ServerPacketReceiverBuilder::build(state_handler, ticker.clone());
    let packet_sender = ServerPacketSenderBuilder::build(ticker.clone());

    let mut server = Server::new(packet_receiver, packet_sender);

    let _ = server.run().await;
}
