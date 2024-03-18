use std::collections::VecDeque;
use std::str::FromStr;

use super::components::shared::vec3d::Vec3d;
use super::packets::move_packet::MovePacket;
use super::packets::packet::Packet;
use super::state_handler::StateHandler;
use super::systems::command_container::CommandContainer;

pub struct PacketHandler<'a> {
    state_handler: &'a StateHandler,
}

impl<'a> PacketHandler<'a> {
    pub fn new(state_handler: &'a StateHandler) -> PacketHandler<'a> {
        PacketHandler { state_handler }
    }

    pub fn consume(&self, packet: Packet) {
        match packet.opcode {
            super::opcode::OpCode::Movement => {
                // ideally this will be extracted
                let world = self.state_handler.get_world();

                // TODO probably an incredibly huge bottleneck

                let mut world = world.lock().unwrap();

                let mut res = world.resource_mut::<CommandContainer<Vec3d>>();

                let packet_data = MovePacket::from_str(&packet.data).unwrap();

                match res.entries.get_mut(&packet_data.entity) {
                    Some(queue) => {
                        queue.push_back(packet_data.vector);
                    }
                    None => {
                        let mut queue: VecDeque<Vec3d> = VecDeque::new();

                        queue.push_back(packet_data.vector);
                    }
                }
            }
            super::opcode::OpCode::Auth => todo!(),
            super::opcode::OpCode::Existence => todo!(),
            super::opcode::OpCode::Spawn => todo!(),
        }
    }
}
