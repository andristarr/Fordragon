use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use bevy_ecs::world::World;
use log::{debug, trace};

use crate::server::{
    commands::move_command::MoveCommand,
    components::{position::Position, shared::vec3d::Vec3d},
    packets::{
        packet::{self, Packet},
        spawn_packet::SpawnPacket,
    },
    systems::command_container::CommandContainer,
};

use super::packet_handler::PacketHandlerTrait;

pub(super) struct SpawnPacketHandler {
    packets: Vec<Packet>,
}

impl SpawnPacketHandler {
    pub fn new() -> Self {
        SpawnPacketHandler { packets: vec![] }
    }
}

impl PacketHandlerTrait for SpawnPacketHandler {
    fn handle_packet(&mut self, packet: Packet) {
        trace!("Handling spawn packet: {:?}", packet);

        self.packets.push(packet);
    }

    fn transform_state(&mut self, world: Arc<RwLock<World>>) {
        debug!(
            "Transforming state with {} spawn packets",
            self.packets.len()
        );

        for packet in &self.packets {
            trace!("Processing spawn packet: {:?}", packet);

            let mut world = world.write().expect("Failed to get write lock world");

            let packet_data = serde_json::from_str::<SpawnPacket>(&packet.data)
                .expect("Failed to deserialize SpawnPacket");

            // TODO simulating spawning X amount of entities instead of one
            for _ in 0..1000 {
                let entity = world
                    .spawn(Position {
                        position: Vec3d::new(
                            packet_data.location.x,
                            packet_data.location.y,
                            packet_data.location.z,
                        ),
                    })
                    .id();

                let mut res = world.resource_mut::<CommandContainer<MoveCommand>>();

                res.entries.insert(entity, VecDeque::new());
            }
        }

        self.packets.clear();
    }
}
