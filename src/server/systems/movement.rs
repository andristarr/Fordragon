use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Query, ResMut},
};

use crate::server::components::{position::Position, shared::vec3d::Vec3d};

use super::command_container::CommandContainer;

pub fn movement_system(
    mut query: Query<(Entity, &mut Position), With<Position>>,
    mut movement_commands: ResMut<CommandContainer<Vec3d>>,
) {
    for (entity, mut position) in query.iter_mut() {
        if let Some(commands) = movement_commands.entries.get_mut(&entity) {
            while let Some(command) = commands.pop_front() {
                position.position.x += command.x;
                position.position.y += command.y;
                position.position.z += command.z;
            }
        }
    }
}
