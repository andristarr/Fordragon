use std::collections::VecDeque;

use bevy_ecs::resource::Resource;

use crate::server::commands::MapableCommand;

#[derive(Resource)]
pub struct UntargetedCommandContainer<T: MapableCommand> {
    pub entries: VecDeque<T>,
}
