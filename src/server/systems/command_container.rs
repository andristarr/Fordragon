use std::collections::{HashMap, VecDeque};

use bevy_ecs::system::Resource;

use crate::server::components::guid::Guid;

#[derive(Resource)]
pub struct CommandContainer<T> {
    pub entries: HashMap<String, VecDeque<T>>
}