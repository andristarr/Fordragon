use serde::{Deserialize, Serialize};

use crate::server::components::shared::vec3d::Vec3d;
#[derive(Debug, Serialize, Deserialize)]
pub struct MovedPacket {
    pub networked_id: String,
    pub vector: Vec3d,
}

impl MovedPacket {
    pub fn new(id: String, vector: Vec3d) -> Self {
        MovedPacket {
            networked_id: id,
            vector,
        }
    }
}
