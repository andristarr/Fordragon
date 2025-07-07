use std::{
    collections::VecDeque,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use bevy_ecs::world::World;
use log::{debug, trace};

use crate::server::{
    commands::move_command::MoveCommand,
    components::{movement_state::MovementState, networked::Networked},
    packets::{packet::Packet, received_packet::ReceivedPacket},
    protocols::recv::move_packet::MovePacket,
    state::authorization_handler::AuthorizationHandlerTrait,
    systems::command_container::CommandContainer,
};

use super::packet_handler::PacketHandlerTrait;

pub(super) struct MovePacketHandler {
    packets: Vec<ReceivedPacket>,
    authorization_handler: Arc<RwLock<dyn AuthorizationHandlerTrait>>,
}

impl MovePacketHandler {
    pub fn new(authorization_handler: Arc<RwLock<dyn AuthorizationHandlerTrait>>) -> Self {
        MovePacketHandler {
            authorization_handler,
            packets: vec![],
        }
    }
}

impl PacketHandlerTrait for MovePacketHandler {
    fn handle_packet(&mut self, addr: SocketAddr, packet: Packet) {
        trace!("Handling move packet: {:?}", packet);

        self.packets.push(ReceivedPacket::new(packet, addr));
    }

    fn transform_state(&mut self, world: Arc<RwLock<World>>) {
        debug!(
            "Transforming state with {} move packets",
            self.packets.len()
        );

        let authorization_handler = self
            .authorization_handler
            .read()
            .expect("Failed to get read lock on authorization handler");

        for packet in &self.packets {
            trace!("Processing move packet: {:?}", packet);

            let mut world = world.write().expect("Failed to get write lock world");

            let character_id = authorization_handler.get_character_id(packet.addr).expect(
                format!(
                    "Failed to get character ID for ({:?}) from authorization handler",
                    packet.addr
                )
                .as_str(),
            );

            let mut res = world.resource_mut::<CommandContainer<MoveCommand>>();

            let packet_data = serde_json::from_str::<MovePacket>(&packet.packet.data)
                .expect("Failed to deserialize MovePacket");

            match res.entries.get_mut(&character_id.to_string()) {
                Some(queue) => {
                    queue.push_back(MoveCommand::new(
                        character_id.to_string(),
                        packet_data.vector.x,
                        packet_data.vector.y,
                        packet_data.vector.z,
                        packet_data.state.clone(),
                    ));
                }
                None => {
                    let mut queue = VecDeque::new();

                    queue.push_back(MoveCommand::new(
                        character_id.to_string().clone(),
                        packet_data.vector.x,
                        packet_data.vector.y,
                        packet_data.vector.z,
                        packet_data.state.clone(),
                    ));
                    res.entries.insert(character_id.to_string(), queue);
                }
            }
        }
    }

    fn clear_packets(&mut self) {
        self.packets.clear();
    }
}
