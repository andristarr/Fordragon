use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Query, ResMut},
};

use crate::server::{commands::move_command::MoveCommand, components::position::Position};

use super::command_container::CommandContainer;

pub fn movement_system(
    mut query: Query<(Entity, &mut Position), With<Position>>,
    mut movement_commands: ResMut<CommandContainer<MoveCommand>>,
) {
    for (entity, mut position) in query.iter_mut() {
        if let Some(commands) = movement_commands.entries.get_mut(&entity) {
            for command in commands {
                // Apply the command to the position
                position.position.x += command.x;
                position.position.y += command.y;
                position.position.z += command.z;
            }
        }
    }
}
