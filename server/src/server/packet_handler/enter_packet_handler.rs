use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
    vec,
};

use bevy_ecs::world::World;
use log::{debug, trace};
use uuid::Uuid;

use crate::server::{
    commands::spawn_command::{EntityComponent, SpawnCommand},
    components::movement_state::MovementStateType,
    packets::{packet::Packet, received_packet::ReceivedPacket},
    protocols::recv::enter_packet::EnterPacket,
    state::authorization_handler::AuthorizationHandlerTrait,
    systems::untargeted_command_container::UntargetedCommandContainer,
};

use super::packet_handler::PacketHandlerTrait;

pub(super) struct EnterPacketHandler {
    packets: Vec<ReceivedPacket>,
    authorization_handler: Arc<RwLock<dyn AuthorizationHandlerTrait>>,
}

impl EnterPacketHandler {
    pub fn new(authorization_handler: Arc<RwLock<dyn AuthorizationHandlerTrait>>) -> Self {
        EnterPacketHandler {
            authorization_handler,
            packets: vec![],
        }
    }
}

impl PacketHandlerTrait for EnterPacketHandler {
    fn handle_packet(&mut self, addr: SocketAddr, packet: Packet) {
        trace!("Handling enter packet: {:?}", packet);

        self.packets.push(ReceivedPacket::new(packet, addr));
    }

    fn transform_state(&mut self, world: Arc<RwLock<World>>) {
        debug!(
            "Transforming state with {} enter packets",
            self.packets.len()
        );

        let mut authorization_handler = self
            .authorization_handler
            .write()
            .expect("Failed to get read lock on authorization handler");

        for packet in &self.packets {
            trace!("Processing enter packet: {:?}", packet);

            let mut world = world.write().expect("Failed to get write lock world");

            let _ = serde_json::from_str::<EnterPacket>(&packet.packet.data)
                .expect("Failed to deserialize Enter Packet");

            let mut res = world.resource_mut::<UntargetedCommandContainer<SpawnCommand>>();

            let character_id = Uuid::new_v4();

            authorization_handler.add_entity(packet.addr.clone(), character_id.clone());

            let cmd = SpawnCommand::new(
                vec![
                    EntityComponent::Position(0.0, 0.0, 0.0),
                    EntityComponent::Networked(character_id.to_string()),
                    EntityComponent::MovementState(MovementStateType::Stopped, 0.25),
                ],
                Some(packet.addr),
            );

            trace!("Adding spawn command: {:?}", cmd);

            res.entries.push_back(cmd);
        }
    }

    fn clear_packets(&mut self) {
        self.packets.clear();
    }
}
