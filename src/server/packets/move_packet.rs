use std::str::FromStr;

use bevy_ecs::entity::Entity;
use serde::{Deserialize, Serialize};

use crate::server::components::shared::vec3d::Vec3d;
#[derive(Debug, Serialize, Deserialize)]
pub struct MovePacket {
    pub entity: Entity,
    pub vector: Vec3d,
}
