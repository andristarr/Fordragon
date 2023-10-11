use crate::server::dispatcher::Dispatcher;
use crate::server::packet::Packet;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

mod common;
mod server;

#[tokio::main]
async fn main() {
    let mut dispatcher = Dispatcher::new(2);

    dispatcher.run(dispatcher.state);
}
