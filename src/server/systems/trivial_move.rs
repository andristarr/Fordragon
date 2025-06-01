use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Query, ResMut},
};
use log::trace;

use crate::server::{
    commands::move_command::MoveCommand,
    components::position::Position,
};

use super::command_container::CommandContainer;

pub fn trivival_move_system(
    mut query: Query<(Entity, &mut Position), With<Position>>,
    mut movement_commands: ResMut<CommandContainer<MoveCommand>>,
) {
    trace!(
        "Running trivial move system for {:?} number of entities",
        query.iter().count()
    );

    for (entity, _position) in query.iter_mut() {
        if let Some(commands) = movement_commands.entries.get_mut(&entity) {
            commands.push_back(MoveCommand::new(entity, 1.0, 0.0, 0.0));
        }
    }

    trace!(
        "Finished trivial move system, movement_commands contains {:?} entries",
        movement_commands.entries.len()
    );
}
