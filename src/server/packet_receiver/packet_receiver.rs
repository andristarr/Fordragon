use log::{debug, warn};

use crate::server::components::position::Position;
use crate::server::components::shared::vec3d::Vec3d;
use crate::server::opcode::OpCode;
use crate::server::packet_handler::packet_handler::{PacketHandler, PacketHandlerTrait};
use crate::server::packets::move_packet::MovePacket;
use crate::server::packets::packet::{self, Packet};
use crate::server::packets::spawn_packet::SpawnPacket;
use crate::server::state;
use crate::server::state::state_handler::{ServerStateHandler, StateHandler};
use crate::server::state::ticker::TickerTrait;
use crate::server::systems::command_container::CommandContainer;
use std::collections::{HashMap, VecDeque};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub trait PacketReceiver {
    fn consume(&self, packet: Packet, addr: SocketAddr);
    fn initialise(&mut self);
    fn inject_packets(
        packet_handler: Arc<Mutex<dyn PacketHandlerTrait>>,
        state: Arc<Mutex<ServerPacketReceiverState<ServerStateHandler>>>,
    );
}

pub struct ServerPacketReceiver {
    ticker: Arc<Mutex<dyn TickerTrait>>,
    state: Arc<Mutex<ServerPacketReceiverState<ServerStateHandler>>>,
    packet_handler: Arc<Mutex<dyn PacketHandlerTrait>>,
}

pub struct ServerPacketReceiverState<T: StateHandler> {
    pub(super) state_handler: T,
    connections: HashMap<SocketAddr, u128>,
}

impl ServerPacketReceiver {
    pub fn new(state_handler: ServerStateHandler, ticker: Arc<Mutex<dyn TickerTrait>>) -> Self {
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
        state: Arc<Mutex<ServerPacketReceiverState<ServerStateHandler>>>,
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
