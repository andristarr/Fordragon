use crate::server::components::position::Position;
use crate::server::components::shared::vec3d::Vec3d;
use crate::server::opcode::OpCode;
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

pub trait PacketHandler {
    fn consume(&self, packet: Packet, addr: SocketAddr);
    fn initialise(&mut self);
    fn inject_packets(handler: Arc<Mutex<ServerPacketHandlerState<ServerStateHandler>>>);
}

pub struct ServerPacketHandler {
    ticker: Arc<Mutex<dyn TickerTrait>>,
    state: Arc<Mutex<ServerPacketHandlerState<ServerStateHandler>>>,
}

pub struct ServerPacketHandlerState<T: StateHandler> {
    pub(super) state_handler: T,
    packets: HashMap<OpCode, Vec<Packet>>,
    connections: HashMap<SocketAddr, u128>,
}

impl ServerPacketHandler {
    pub fn new(state_handler: ServerStateHandler, ticker: Arc<Mutex<dyn TickerTrait>>) -> Self {
        let state = ServerPacketHandlerState {
            state_handler,
            packets: HashMap::new(),
            connections: HashMap::new(),
        };

        let state = Arc::new(Mutex::new(state));

        ServerPacketHandler { ticker, state }
    }
}

impl PacketHandler for ServerPacketHandler {
    fn inject_packets(state: Arc<Mutex<ServerPacketHandlerState<ServerStateHandler>>>) {
        let world = state.lock().unwrap().state_handler.get_world();

        for (opcode, packets) in state.lock().unwrap().packets.iter() {
            for packet in packets {
                match opcode {
                    OpCode::Movement => {
                        // TODO probably an incredibly huge bottleneck

                        let mut world = world.write().unwrap();

                        let mut res = world.resource_mut::<CommandContainer<Vec3d>>();

                        let packet_data = serde_json::from_str::<MovePacket>(&packet.data).unwrap();

                        match res.entries.get_mut(&packet_data.entity) {
                            Some(queue) => {
                                queue.push_back(packet_data.vector);
                            }
                            None => {
                                let mut queue: VecDeque<Vec3d> = VecDeque::new();

                                queue.push_back(packet_data.vector);

                                res.entries.insert(packet_data.entity, queue);
                            }
                        }
                    }
                    OpCode::Spawn => {
                        let mut world = world.write().unwrap();

                        let packet_data =
                            serde_json::from_str::<SpawnPacket>(&packet.data).unwrap();

                        for _ in 0..1000 {
                            let entity = world
                                .spawn(Position {
                                    position: Vec3d {
                                        x: packet_data.location.x,
                                        y: packet_data.location.y,
                                        z: packet_data.location.z,
                                    },
                                })
                                .id();

                            let mut res = world.resource_mut::<CommandContainer<Vec3d>>();

                            res.entries.insert(entity, VecDeque::new());
                        }
                    }
                    _ => panic!("Unknown opcode"),
                }
            }
        }

        state.lock().unwrap().packets.clear();
    }

    fn consume(&self, packet: Packet, addr: SocketAddr) {
        if self.state.lock().unwrap().connections.get(&addr).is_none() {
            self.state
                .lock()
                .unwrap()
                .connections
                .insert(addr, packet.id);
        } else {
            if self.state.lock().unwrap().connections.get(&addr).unwrap() > &packet.id {
                println!("Packet loss detected, dropping packet...");
                return;
            } else {
                self.state
                    .lock()
                    .unwrap()
                    .connections
                    .insert(addr, packet.id);
            }
        }

        match packet.opcode {
            OpCode::Movement => {
                if self
                    .state
                    .lock()
                    .unwrap()
                    .packets
                    .get(&OpCode::Movement)
                    .is_some()
                {
                    self.state
                        .lock()
                        .unwrap()
                        .packets
                        .get_mut(&OpCode::Movement)
                        .unwrap()
                        .push(packet);
                } else {
                    self.state
                        .lock()
                        .unwrap()
                        .packets
                        .insert(OpCode::Movement, vec![packet]);
                }
            }
            OpCode::Auth => todo!(),
            OpCode::Spawn => {
                if self
                    .state
                    .lock()
                    .unwrap()
                    .packets
                    .get(&OpCode::Spawn)
                    .is_some()
                {
                    self.state
                        .lock()
                        .unwrap()
                        .packets
                        .get_mut(&OpCode::Spawn)
                        .unwrap()
                        .push(packet);
                } else {
                    self.state
                        .lock()
                        .unwrap()
                        .packets
                        .insert(OpCode::Spawn, vec![packet]);
                }
            }
        }
    }

    fn initialise(&mut self) {
        let state = self.state.clone();

        self.ticker.lock().unwrap().register(Box::new(move || {
            println!("Injecting packets...");

            ServerPacketHandler::inject_packets(state.clone());
        }));

        self.state.lock().unwrap().state_handler.start();
    }
}
