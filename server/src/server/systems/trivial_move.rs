use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Query, ResMut},
};
use log::trace;

use crate::server::{
    commands::move_command::MoveCommand,
    components::{networked::Networked, position::Position},
};

use super::command_container::CommandContainer;

pub fn trivival_move_system(
    mut query: Query<(Entity, &mut Position, &mut Networked), (With<Position>, With<Networked>)>,
    mut movement_commands: ResMut<CommandContainer<MoveCommand>>,
) {
    trace!(
        "Running trivial move system for {:?} number of entities",
        query.iter().count()
    );

    for (_, _position, networked) in query.iter_mut() {
        if let Some(commands) = movement_commands.entries.get_mut(&networked.id) {
            commands.push_back(MoveCommand::new(networked.id.clone(), 1.0, 0.0, 0.0));
        }
    }

    trace!(
        "Finished trivial move system, movement_commands contains {:?} entries",
        movement_commands.entries.len()
    );
}
