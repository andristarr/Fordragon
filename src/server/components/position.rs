use bevy_ecs::component::Component;

use super::shared::vec3d::Vec3d;

#[derive(Component)]
pub struct Position {
    pub position: Vec3d,
}
