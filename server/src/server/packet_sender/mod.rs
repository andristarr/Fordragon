use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

pub mod builder;
pub mod packet_sender;
pub mod send_packet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TargetAddress {
    Broadcast,
    Targeted(Vec<SocketAddr>),
}
