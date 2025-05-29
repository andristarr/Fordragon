use log::{debug, warn};

use crate::server::packet_handler::packet_handler::{PacketHandler, PacketHandlerTrait};
use crate::server::packets::packet::Packet;
use crate::server::state::state_handler::StateHandler;
use crate::server::state::ticker::TickerTrait;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub trait PacketReceiver: Send + Sync {
    fn consume(&self, packet: Packet, addr: SocketAddr);
    fn initialise(&mut self);
    fn inject_packets(
        packet_handler: Arc<Mutex<dyn PacketHandlerTrait>>,
        state: Arc<Mutex<ServerPacketReceiverState>>,
    );
}

pub struct ServerPacketReceiver {
    ticker: Arc<Mutex<dyn TickerTrait>>,
    state: Arc<Mutex<ServerPacketReceiverState>>,
    packet_handler: Arc<Mutex<dyn PacketHandlerTrait>>,
}

pub struct ServerPacketReceiverState {
    pub(super) state_handler: Box<dyn StateHandler>,
    connections: HashMap<SocketAddr, u128>,
}

impl ServerPacketReceiver {
    pub fn new(state_handler: Box<dyn StateHandler>, ticker: Arc<Mutex<dyn TickerTrait>>) -> Self {
        let state = ServerPacketReceiverState {
            state_handler,
            connections: HashMap::new(),
        };

        let state = Arc::new(Mutex::new(state));

        ServerPacketReceiver {
            ticker,
            state,
            packet_handler: Arc::new(Mutex::new(PacketHandler::new())),
        }
    }
}

impl PacketReceiver for ServerPacketReceiver {
    fn inject_packets(
        packet_handler: Arc<Mutex<dyn PacketHandlerTrait>>,
        state: Arc<Mutex<ServerPacketReceiverState>>,
    ) {
        packet_handler
            .lock()
            .expect(("Failed to lock packet handler"))
            .transform_state(
                state
                    .lock()
                    .expect("Failed to lock packet receiver state")
                    .state_handler
                    .get_world(),
            );
    }

    fn consume(&self, packet: Packet, addr: SocketAddr) {
        if self.state.lock().unwrap().connections.get(&addr).is_none() {
            self.state
                .lock()
                .unwrap()
                .connections
                .insert(addr, packet.id.unwrap_or(0));
        } else {
            if self.state.lock().unwrap().connections.get(&addr).unwrap() > &packet.id.unwrap_or(0)
            {
                warn!("Packet loss detected, dropping packet...");
                return;
            } else {
                self.state
                    .lock()
                    .unwrap()
                    .connections
                    .insert(addr, packet.id.unwrap_or(0));
            }
        }

        self.packet_handler
            .lock()
            .expect("Failed to lock packet handler")
            .handle_packet(packet);
    }

    fn initialise(&mut self) {
        let state = self.state.clone();
        let packet_handler = self.packet_handler.clone();

        self.ticker.lock().unwrap().register(Box::new(move || {
            debug!("Injecting packets...");

            ServerPacketReceiver::inject_packets(packet_handler.clone(), state.clone());
        }));

        self.state.lock().unwrap().state_handler.start();
    }
}
