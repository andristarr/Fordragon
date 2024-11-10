use crate::server::components::position::Position;
use crate::server::components::shared::vec3d::Vec3d;
use crate::server::opcode::OpCode;
use crate::server::packets::move_packet::MovePacket;
use crate::server::packets::packet::Packet;
use crate::server::packets::spawn_packet::SpawnPacket;
use crate::server::state::state_handler::StateHandler;
use crate::server::systems::command_container::CommandContainer;
use std::collections::VecDeque;
use std::str::FromStr;

pub trait PacketHandler {
    fn consume(&self, packet: Packet);
    fn initialise(&mut self);
}

pub struct ServerPacketHandler<T: StateHandler> {
    pub(super) state_handler: T,
}

impl<T: StateHandler> ServerPacketHandler<T> {
    pub fn new(state_handler: T) -> Self {
        ServerPacketHandler { state_handler }
    }
}

impl<T: StateHandler> PacketHandler for ServerPacketHandler<T> {
    fn consume(&self, packet: Packet) {
        match packet.opcode {
            OpCode::Movement => {
                // ideally this will be extracted
                let world = self.state_handler.get_world();

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
            OpCode::Auth => todo!(),
            OpCode::Spawn => {
                // ideally this will be extracted
                let world = self.state_handler.get_world();

                // TODO probably an incredibly huge bottleneck
                {
                    let mut world = world.write().unwrap();

                    let packet_data = serde_json::from_str::<SpawnPacket>(&packet.data).unwrap();

                    let entity = world
                        .spawn(Position {
                            position: Vec3d {
                                x: packet_data.location.x,
                                y: packet_data.location.y,
                                z: packet_data.location.z,
                            },
                        })
                        .id();

                    println!("Spawned entity: {:?}", entity);

                    let mut res = world.resource_mut::<CommandContainer<Vec3d>>();

                    res.entries.insert(entity, VecDeque::new());
                }
            }
        }
    }

    fn initialise(&mut self) {
        self.state_handler.start();
    }
}
