use serde::{Deserialize, Serialize};

use crate::server::{
    commands::MapableCommand, components::shared::vec3d::Vec3d, packets::move_packet::MovePacket,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveCommand {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl MoveCommand {
    pub fn new(id: String, x: f64, y: f64, z: f64) -> Self {
        MoveCommand { id, x, y, z }
    }
}

impl MapableCommand for MoveCommand {
    type PacketType = MovePacket;

    fn map_to_packet(&self) -> Self::PacketType {
        MovePacket {
            id: self.id.clone(),
            vector: Vec3d::new(self.x, self.y, self.z),
        }
    }
}
