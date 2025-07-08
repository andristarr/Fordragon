use std::sync::{Arc, Mutex, RwLock};

use bevy_ecs::world::World;
use log::{debug, trace};
use serde::{Deserialize, Serialize};

use crate::server::{
    commands::{MapableCommand, StateMappedCommand},
    components::{
        movement_state::MovementStateType, networked::Networked, position::Position,
        shared::vec3d::Vec3d,
    },
    opcode::OpCode,
    packet_sender::{
        packet_sender::{PacketSender, ServerPacketSender},
        send_packet::SendPacket,
        TargetAddress,
    },
    protocols::send::{enown_packet::EnownPacket, spawn_packet::SpawnPacket},
    systems::untargeted_command_container::UntargetedCommandContainer,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityComponent {
    Position(f64, f64, f64),
    Networked(String),
    MovementState(MovementStateType, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnCommand {
    pub components: Vec<EntityComponent>,
    pub owning_connection: TargetAddress,
}

impl SpawnCommand {
    pub fn new(components: Vec<EntityComponent>, owning_connection: TargetAddress) -> Self {
        SpawnCommand {
            components,
            owning_connection,
        }
    }
}

impl MapableCommand for SpawnCommand {
    type PacketType = SpawnPacket;

    fn map_to_packet(&self, _world: &mut World) -> Self::PacketType {
        let networked = self
            .components
            .iter()
            .find_map(|c| {
                if let EntityComponent::Networked(id) = c {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        SpawnPacket {
            location: Vec3d::new(0.0, 0.0, 0.0),
            id: networked,
        }
    }
}

impl StateMappedCommand for SpawnCommand {
    fn map(world: Arc<RwLock<World>>, sender: Arc<Mutex<ServerPacketSender>>) {
        let mut world = world.write().expect("Failed to get write lock to world");
        let sender = sender.lock().expect("Failed to lock sender");

        debug!(
            "Enqueuing packets from {:?} spawn commands",
            world
                .resource_mut::<UntargetedCommandContainer<SpawnCommand>>()
                .entries
                .len()
        );

        // Also queue already existing packets

        let networked_entities = world
            .query::<(&Networked, &Position)>()
            .iter(&world)
            .map(|(networked, position)| SpawnPacket {
                id: networked.id.clone(),
                location: Vec3d::new(
                    position.position.x,
                    position.position.y,
                    position.position.z,
                ),
            })
            .collect::<Vec<SpawnPacket>>();

        let commands: Vec<_> = world
            .resource_mut::<UntargetedCommandContainer<SpawnCommand>>()
            .entries
            .drain(..)
            .collect();

        for command in commands {
            trace!("Processing command: {:?}", command);

            let packet = command.map_to_packet(&mut world);

            let packet_data =
                serde_json::to_string(&packet).expect("Failed to serialize SpawnCommand");

            sender.enqueue(SendPacket::new(
                packet_data.clone(),
                OpCode::Spawn,
                TargetAddress::Broadcast,
            ));

            networked_entities
                .iter()
                .filter(|n| n.id != packet.id)
                .for_each(|spawn| {
                    sender.enqueue(SendPacket::new(
                        serde_json::to_string(&spawn).expect("Failed to serialize SpawnPacket"),
                        OpCode::Spawn,
                        TargetAddress::Broadcast,
                    ));
                });

            let enown_packet = EnownPacket {
                id: packet.id.clone(),
            };

            sender.enqueue(SendPacket::new(
                serde_json::to_string(&enown_packet).expect("Failed to serialize EnownPacket"),
                OpCode::Enown,
                command.owning_connection.clone(),
            ));
        }
    }
}
