use serde::{Deserialize, Serialize};

use crate::server::components::{movement_state::MovementStateType, shared::vec3d::Vec3d};
#[derive(Debug, Serialize, Deserialize)]
pub struct MovePacket {
    pub vector: Vec3d,
    pub state: MovementStateType,
}

impl MovePacket {
    pub fn new(vector: Vec3d, state: MovementStateType) -> Self {
        MovePacket { vector, state }
    }
}
