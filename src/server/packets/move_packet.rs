use std::str::FromStr;

use bevy_ecs::entity::Entity;
use serde::{Deserialize, Serialize};

use crate::server::components::shared::vec3d::Vec3d;
#[derive(Debug, Serialize, Deserialize)]
pub struct MovePacket {
    pub entity: Entity,
    pub vector: Vec3d,
}

impl FromStr for MovePacket {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
