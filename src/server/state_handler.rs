use bevy_ecs::{schedule::Schedule, world::World};

pub struct StateHandler {
    world: World,
    schedule: Schedule,
}

impl StateHandler {
    pub fn new() -> StateHandler {
        StateHandler {
            world: World::default(),
            schedule: Schedule::default(),
        }
    }
}
