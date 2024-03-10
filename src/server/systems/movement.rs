use bevy_ecs::system::{Query, Res};

use crate::server::components::{guid::Guid, position::Vec3d};

use super::command_container::CommandContainer;

pub fn movement_system(guids: Query<&Guid>, movement_commands: Res<CommandContainer<Vec3d>>) {
    for guid in &guids {
        let commands = movement_commands.entries.get(&guid.guid);

        if let Some(commands) = commands {
            for command in commands {
                
            }
        }
    }
}