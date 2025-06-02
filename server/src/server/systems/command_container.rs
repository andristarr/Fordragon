use std::collections::{HashMap, VecDeque};

use bevy_ecs::{entity::Entity, system::Resource};

#[derive(Resource)]
pub struct CommandContainer<T> {
    pub entries: HashMap<Entity, VecDeque<T>>,
}
