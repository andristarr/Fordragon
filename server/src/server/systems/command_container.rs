use std::collections::{HashMap, VecDeque};

use bevy_ecs::resource::Resource;

use crate::server::commands::MapableCommand;

#[derive(Resource)]
pub struct CommandContainer<T: MapableCommand> {
    pub entries: HashMap<String, VecDeque<T>>,
}
