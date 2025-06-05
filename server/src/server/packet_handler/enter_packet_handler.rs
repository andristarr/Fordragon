use std::{
    sync::{Arc, RwLock},
    vec,
};

use bevy_ecs::world::World;
use log::{debug, trace};
use uuid::Uuid;

use crate::server::{
    commands::spawn_command::{EntityComponent, SpawnCommand},
    packets::{enter_packet::EnterPacket, packet::Packet},
    systems::untargeted_command_container::UntargetedCommandContainer,
};

use super::packet_handler::PacketHandlerTrait;

pub(super) struct EnterPacketHandler {
    packets: Vec<Packet>,
}

impl EnterPacketHandler {
    pub fn new() -> Self {
        EnterPacketHandler { packets: vec![] }
    }
}

impl PacketHandlerTrait for EnterPacketHandler {
    fn handle_packet(&mut self, packet: Packet) {
        trace!("Handling enter packet: {:?}", packet);

        self.packets.push(packet);
    }

    fn transform_state(&mut self, world: Arc<RwLock<World>>) {
        debug!(
            "Transforming state with {} enter packets",
            self.packets.len()
        );

        for packet in &self.packets {
            trace!("Processing enter packet: {:?}", packet);

            let mut world = world.write().expect("Failed to get write lock world");

            let _ = serde_json::from_str::<EnterPacket>(&packet.data)
                .expect("Failed to deserialize Enter Packet");

            let mut res = world.resource_mut::<UntargetedCommandContainer<SpawnCommand>>();

            let cmd = SpawnCommand::new(vec![
                EntityComponent::Position(0.0, 0.0, 0.0),
                EntityComponent::Networked(Uuid::new_v4().to_string()),
            ]);

            trace!("Adding spawn command: {:?}", cmd);

            res.entries.push_back(cmd);
        }
    }

    fn clear_packets(&mut self) {
        self.packets.clear();
    }
}
