use serde::{Deserialize, Serialize};

use crate::server::components::shared::vec3d::Vec3d;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpawnPacket {
    pub spawned_type: String,
    pub location: Vec3d,
}
