use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Query, ResMut},
};
use log::debug;

use crate::server::{
    commands::move_command::MoveCommand,
    components::{networked::Networked, position::Position},
};

use super::command_container::CommandContainer;

pub fn movement_system(
    mut query: Query<(Entity, &mut Position, &mut Networked), (With<Position>, With<Networked>)>,
    mut movement_commands: ResMut<CommandContainer<MoveCommand>>,
) {
    for (_, mut position, networked) in query.iter_mut() {
        if let Some(commands) = movement_commands.entries.get_mut(&networked.id) {
            for command in commands {
                // Apply the command to the position
                position.position.x = command.x;
                position.position.y = command.y;
                position.position.z = command.z;

                debug!(
                    "Entity {} moved to position: ({}, {}, {})",
                    networked.id, position.position.x, position.position.y, position.position.z
                );
            }
        }
    }
}
