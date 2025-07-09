use std::time::Instant;

use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Query, Res, ResMut},
};
use log::{debug, trace};

use crate::server::{
    commands::moved_command::MovedCommand,
    components::{
        movement_state::{MovementState, MovementStateType},
        networked::Networked,
        position::Position,
    },
    systems::delta_time::DeltaTime,
};

use super::command_container::CommandContainer;

pub fn movement_system(
    mut query: Query<
        (Entity, &mut Position, &mut Networked, &mut MovementState),
        (With<Position>, With<Networked>, With<MovementState>),
    >,
    mut moved_commands: ResMut<CommandContainer<MovedCommand>>,
    delta_time: Res<DeltaTime>,
) {
    let delta = delta_time.current();
    let now = Instant::now();

    // get the elapsed milliseconds since delta
    let elapsed_secs = now.duration_since(delta).as_millis() as f64 / 1000.0;

    // TODO: Currently this does not handle invalidation of weird directions (eg, no collision detection)
    for (_, mut position, networked, movement_state) in query.iter_mut() {
        trace!(
            "Processing movement for entity: {}, current state is: {:?}",
            networked.id,
            movement_state.current_state
        );
        if movement_state.current_state == MovementStateType::Stopped {
            debug!("Entity {} is stopped, skipping movement", networked.id);
            continue;
        }

        position.position.x += movement_state.direction.x * movement_state.velocity * elapsed_secs;
        position.position.y += movement_state.direction.y * movement_state.velocity * elapsed_secs;
        position.position.z += movement_state.direction.z * movement_state.velocity * elapsed_secs;

        // Add to moved_commands or create a new one if it doesn't exist
        moved_commands
            .entries
            .entry(networked.id.clone())
            .or_default()
            .push_back(MovedCommand::new(
                networked.id.clone(),
                position.position.x,
                position.position.y,
                position.position.z,
            ));
    }
}
