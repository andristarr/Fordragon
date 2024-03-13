use bevy_ecs::{entity::Entity, query::With, system::{Query, Res, ResMut}};

use crate::server::components::position::Position;

use super::command_container::CommandContainer;

pub fn movement_system(mut query: Query<(Entity, &mut Position), With<Position>>, mut movement_commands: ResMut<CommandContainer<Position>>) {
    for (entity, mut position) in query.iter_mut() {
        if let Some(commands) = movement_commands.entries.get_mut(&entity) {
            loop {
                if let Some(command) = commands.pop_front()
                {
                    position.position.x += command.position.x;
                    position.position.y += command.position.y;
                    position.position.z += command.position.z;
                }
            }
        }
    }
}