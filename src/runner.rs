#![warn(unused_extern_crates)]

use std::sync::{Arc, Mutex};

use bevy_ecs::{schedule::Schedule, world::World};
use server::{
    packet_handler::{builder::ServerPacketHandlerBuilder, packet_handler::ServerPacketHandler},
    server::Server,
    state::{
        state_handler::{ServerStateHandler, StateHandler},
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
