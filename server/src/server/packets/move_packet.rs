use serde::{Deserialize, Serialize};

use crate::server::components::shared::vec3d::Vec3d;
#[derive(Debug, Serialize, Deserialize)]
pub struct MovePacket {
    pub id: String,
    pub vector: Vec3d,
}

impl MovePacket {
    pub fn new(id: String, vector: Vec3d) -> Self {
        MovePacket { id, vector }
    }
}
