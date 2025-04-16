use std::{net::SocketAddr, sync::{Arc, Mutex}};

use crate::server::packets::packet::Packet;

pub struct ServerPacketSenderState{
    pub packets: Vec<Packet>,
    pub connections: Vec<SocketAddr>,
}

// this is the trivial implementation where everything gets broadcasted to everyone
pub trait PacketSender {
    fn send(&self, packet: Packet, addr: SocketAddr);
    fn initialise(&mut self);
    fn inject_packets(state: Arc<Mutex<ServerPacketSenderState>>);
} 