use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

use crate::server::components::shared::vec3d::Vec3d;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MovementStateType {
    Moving,
    Stopped,
}

#[derive(Component, Debug, Clone)]
pub struct MovementState {
    pub current_state: MovementStateType,
    pub velocity: f64,
    pub direction: Vec3d,
}
