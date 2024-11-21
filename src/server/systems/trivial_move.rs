use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Query, ResMut},
};

use crate::server::components::{position::Position, shared::vec3d::Vec3d};

use super::command_container::CommandContainer;

pub fn trivival_move_system(
    mut query: Query<(Entity, &mut Position), With<Position>>,
    mut movement_commands: ResMut<CommandContainer<Vec3d>>,
) {
    println!(
        "Running trivial move system for {:?} number of entities",
        query.iter().count()
    );

    for (entity, _position) in query.iter_mut() {
        if let Some(commands) = movement_commands.entries.get_mut(&entity) {
            commands.push_back(Vec3d {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            });
        }
    }
}
