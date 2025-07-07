use bevy_ecs::world::World;
use serde::{Deserialize, Serialize};

use crate::server::{
    commands::MapableCommand,
    components::{networked::Networked, position::Position, shared::vec3d::Vec3d},
    protocols::send::moved_packet::MovedPacket,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovedCommand {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl MovedCommand {
    pub fn new(id: String, x: f64, y: f64, z: f64) -> Self {
        MovedCommand { id, x, y, z }
    }
}

impl MapableCommand for MovedCommand {
    type PacketType = MovedPacket;

    fn map_to_packet(&self, world: &mut World) -> Self::PacketType {
        let current_position = world
            .query::<(&Position, &Networked)>()
            .iter(&world)
            .find(|(_, networked)| networked.id == self.id)
            .map(|(position, _)| position.position.clone())
            .expect(&format!("Entity with ID {} not found", self.id));

        MovedPacket {
            networked_id: self.id.clone(),
            vector: Vec3d::new(current_position.x, current_position.y, current_position.z),
        }
    }
}
