use bevy_ecs::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveCommand {
    pub entity: Entity,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl MoveCommand {
    pub fn new(entity: Entity, x: f64, y: f64, z: f64) -> Self {
        MoveCommand { entity, x, y, z }
    }
}
