use serde::{Deserialize, Serialize};

use crate::server::{
    commands::MapableCommand, components::shared::vec3d::Vec3d, packets::spawn_packet::SpawnPacket,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityComponent {
    Position(f64, f64, f64),
    Networked(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnCommand {
    pub components: Vec<EntityComponent>,
}

impl SpawnCommand {
    pub fn new(components: Vec<EntityComponent>) -> Self {
        SpawnCommand { components }
    }
}

impl MapableCommand for SpawnCommand {
    type PacketType = SpawnPacket;

    fn map_to_packet(&self) -> Self::PacketType {
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
