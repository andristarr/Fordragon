use bevy_ecs::system::{Commands, ResMut};

use crate::server::{
    commands::spawn_command::{EntityComponent, SpawnCommand},
    components::{
        movement_state::MovementState, networked::Networked, position::Position,
        shared::vec3d::Vec3d,
    },
    systems::untargeted_command_container::UntargetedCommandContainer,
};

pub fn enter_world_system(
    mut commands: Commands,
    spawn_commands: ResMut<UntargetedCommandContainer<SpawnCommand>>,
) {
    for spawn_command in spawn_commands.entries.iter() {
        let mut entity = commands.spawn_empty();

        for component in &spawn_command.components {
            match component {
                EntityComponent::Position(x, y, z) => {
                    entity.insert(Position {
                        position: Vec3d::new(*x, *y, *z),
                    });
                }
                EntityComponent::Networked(id) => {
                    entity.insert(Networked { id: id.clone() });
                }
                EntityComponent::MovementState(state, velocity) => {
                    entity.insert(MovementState {
                        current_state: state.clone(),
                        velocity: *velocity,
                        direction: Vec3d::zero(),
                    });
                }
            }
        }
    }
}
