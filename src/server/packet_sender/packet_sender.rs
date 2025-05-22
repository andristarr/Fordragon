use std::{
    collections::HashSet,
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
};

use crate::server::{packets::packet::Packet, server::Server};

pub struct ServerPacketSenderState {
    pub packets: Vec<Packet>,
    pub connections: HashSet<Arc<SocketAddr>>,
}

// this is the trivial implementation where everything gets broadcasted to everyone
pub trait PacketSender {
    fn try_register(&mut self, addr: SocketAddr);
    fn send(&self, packet: Packet, addr: SocketAddr);
    fn initialise(&mut self);
    fn inject_packets(state: Arc<Mutex<ServerPacketSenderState>>);
}

pub struct ServerPacketSender {
    pub state: Arc<Mutex<ServerPacketSenderState>>,
}

impl ServerPacketSender {
    pub fn new() -> Self {
        let state = ServerPacketSenderState {
            packets: Vec::new(),
            connections: HashSet::new(),
        };

        let state = Arc::new(Mutex::new(state));

        ServerPacketSender { state }
    }
}

impl PacketSender for ServerPacketSender {
    fn try_register(&mut self, addr: SocketAddr) {
        println!("Registering address {:?}", addr);

        let mut state = self.state.lock().unwrap();

        if !state.connections.contains(&Arc::new(addr)) {
            state.connections.insert(Arc::new(addr));
        }
    }

    fn send(&self, packet: Packet, addr: SocketAddr) {
        println!("Sending packet to {:?}", addr);
    }

    fn initialise(&mut self) {
        // TODO implement this
        println!("Initialising packet sender");
    }

    fn inject_packets(state: Arc<Mutex<ServerPacketSenderState>>) {
        // TODO implement this
        println!("Injecting packets");
    }
}
