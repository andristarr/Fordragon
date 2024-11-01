#![warn(unused_extern_crates)]

use std::sync::{Arc, Mutex};

use server::{
    packet_handler::builder::ServerPacketHandlerBuilder,
    server::Server,
    state::{
        state_handler::ServerStateHandler,
        ticker::Ticker,
    },
};

mod common;
mod server;

#[tokio::main]
async fn main() {
    let ticker = Ticker::new(8);

    let state_handler = ServerStateHandler::new(Arc::new(Mutex::new(ticker)));

    let packet_handler = ServerPacketHandlerBuilder::build(state_handler);

    let mut server = Server::new(Box::new(packet_handler));

    let _ = server.run().await;
}
