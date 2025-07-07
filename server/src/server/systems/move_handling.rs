use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Query, ResMut},
};
use log::{debug, info};

use crate::server::{
    commands::move_command::MoveCommand,
    components::{
        movement_state::MovementState, networked::Networked, position::Position,
        shared::vec3d::Vec3d,
    },
};

use super::command_container::CommandContainer;

pub fn move_handling_system(
    mut query: Query<
        (Entity, &mut Networked, &mut MovementState),
        (With<Position>, With<Networked>, With<MovementState>),
    >,
    mut move_command: ResMut<CommandContainer<MoveCommand>>,
) {
    for (_, networked, mut movement_state) in query.iter_mut() {
        if let Some(commands) = move_command.entries.get_mut(&networked.id) {
            for command in commands {
                debug!(
                    "Entity {} movement state is {:?}",
                    networked.id, command.state
                );

                movement_state.current_state = command.state.clone();

                movement_state.direction = Vec3d::new(command.x, command.y, command.z);
            }
        }
    }
}
