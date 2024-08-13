#![warn(unused_extern_crates)]

use std::sync::{Arc, Mutex};

use bevy_ecs::{schedule::Schedule, world::World};
use server::{packet_handler::packet_handler::ServerPacketHandler, server::Server, state_handler::state_handler::{ServerStateHandler, StateHandler}};

mod common;
mod server;

#[tokio::main]
async fn main() {
    let state_handler = ServerStateHandler::new();

    let packet_handler = ServerPacketHandler::new(Box::new(state_handler));
    
    let mut server = Server::new(Box::new(packet_handler));

    let _ = server.run().await;
}
