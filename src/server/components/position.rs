use bevy_ecs::component::Component;

use crate::common::model::Vec3d;

#[derive(Component)]
pub struct Position {
    pub position: Vec3d
}

