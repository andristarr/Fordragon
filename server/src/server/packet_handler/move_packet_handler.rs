use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use bevy_ecs::world::World;
use log::{debug, trace};

use crate::server::{
    commands::move_command::MoveCommand,
    packets::{move_packet::MovePacket, packet::Packet},
    systems::command_container::CommandContainer,
};

use super::packet_handler::PacketHandlerTrait;

pub(super) struct MovePacketHandler {
    packets: Vec<Packet>,
}

impl MovePacketHandler {
    pub fn new() -> Self {
        MovePacketHandler { packets: vec![] }
    }
}

impl PacketHandlerTrait for MovePacketHandler {
    fn handle_packet(&mut self, packet: Packet) {
        trace!("Handling move packet: {:?}", packet);

        self.packets.push(packet);
    }

    fn transform_state(&mut self, _world: Arc<RwLock<World>>) {
        debug!(
            "Transforming state with {} move packets",
            self.packets.len()
        );

        for packet in &self.packets {
            trace!("Processing move packet: {:?}", packet);

            let mut world = _world.write().expect("Failed to get write lock world");

            let mut res = world.resource_mut::<CommandContainer<MoveCommand>>();

            let packet_data = serde_json::from_str::<MovePacket>(&packet.data)
                .expect("Failed to deserialize MoveCommand");

            match res.entries.get_mut(&packet_data.id) {
                Some(queue) => {
                    queue.push_back(MoveCommand::new(
                        packet_data.id,
                        packet_data.vector.x,
                        packet_data.vector.y,
                        packet_data.vector.z,
                    ));
                }
                None => {
                    let mut queue = VecDeque::new();

                    queue.push_back(MoveCommand::new(
                        packet_data.id.clone(),
                        packet_data.vector.x,
                        packet_data.vector.y,
                        packet_data.vector.z,
                    ));
                    res.entries.insert(packet_data.id, queue);
                }
            }
        }
    }

    fn clear_packets(&mut self) {
        self.packets.clear();
    }
}
