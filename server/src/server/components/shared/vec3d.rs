use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3d { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}
