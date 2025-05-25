use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use log::{debug, error, info};
use tokio::net::UdpSocket;

use crate::server::{packets::packet::Packet, server::Server, state::ticker::TickerTrait};

pub struct ServerPacketSenderState {
    pub packets: Vec<Packet>,
    pub connections: HashSet<Arc<SocketAddr>>,
    pub socket: Option<Arc<UdpSocket>>,
}

// this is the trivial implementation where everything gets broadcasted to everyone
pub trait PacketSender: Send + Sync {
    fn try_register(&mut self, addr: SocketAddr);
    fn enqueue(&self, packet: Packet);
    fn initialise(&mut self, socket: Arc<UdpSocket>);
    fn emit_packets(state: Arc<Mutex<ServerPacketSenderState>>);
}

pub struct ServerPacketSender {
    pub state: Arc<Mutex<ServerPacketSenderState>>,
    ticker: Arc<Mutex<dyn TickerTrait>>,
}

impl ServerPacketSender {
    pub fn new(ticker: Arc<Mutex<dyn TickerTrait>>) -> Self {
        let state = ServerPacketSenderState {
            packets: Vec::new(),
            connections: HashSet::new(),
            socket: None,
        };

        let state = Arc::new(Mutex::new(state));

        ServerPacketSender { state, ticker }
    }
}

impl PacketSender for ServerPacketSender {
    fn try_register(&mut self, addr: SocketAddr) {
        debug!("Registering address {:?}", addr);

        let mut state = self.state.lock().unwrap();

        if !state.connections.contains(&Arc::new(addr)) {
            state.connections.insert(Arc::new(addr));
        }
    }

    fn enqueue(&self, packet: Packet) {
        debug!("Sending packet {:?}", packet);

        let mut state = self.state.lock().unwrap();
        state.packets.push(packet);
    }

    fn initialise(&mut self, socket: Arc<UdpSocket>) {
        info!("Initialising packet sender");

        let mut state = self.state.lock().unwrap();

        state.socket = Some(socket.clone());

        let state = self.state.clone();

        self.ticker.lock().unwrap().register(Box::new(move || {
            // Emit packets every tick
            ServerPacketSender::emit_packets(state.clone());
        }));
    }

    fn emit_packets(state: Arc<Mutex<ServerPacketSenderState>>) {
        let (packets, connections, socket) = {
            let mut state = state.lock().unwrap();
            let packets = std::mem::take(&mut state.packets);
            let connections = state.connections.clone();
            let socket = state.socket.clone().unwrap();
            (packets, connections, socket)
        };

        debug!("Emiting {:?} packets", packets.len());

        tokio::spawn(async move {
            use futures::future::join_all;

            let mut send_futures = Vec::new();

            for packet in packets {
                let bytes = match serde_json::to_vec(&packet) {
                    Ok(b) => b,
                    Err(e) => {
                        error!("Failed to serialize packet: {:?}", e);
                        continue;
                    }
                };

                for addr in &connections {
                    let socket = socket.clone();
                    let addr = addr.clone();
                    let bytes = bytes.clone();

                    let fut = async move {
                        if let Err(e) = socket.send_to(&bytes, &*addr).await {
                            error!("Failed to send packet: {:?}", e);
                        }
                    };
                    send_futures.push(fut);
                }
            }

            join_all(send_futures).await;
        });
    }
}
