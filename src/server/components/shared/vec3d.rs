use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Debug)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
