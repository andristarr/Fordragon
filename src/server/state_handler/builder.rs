use std::sync::{Arc, Mutex};

use bevy_ecs::{schedule::Schedule, world::World};

use super::state_handler::{ServerStateHandler, StateHandler};

pub trait StateHandlerBuilder {
    fn build(&self) -> impl StateHandler;
}

pub struct ServerStateHandlerBuilder;

impl StateHandlerBuilder for ServerStateHandlerBuilder {
    fn build(&self) -> impl StateHandler {
        ServerStateHandler {
            world: Arc::new(Mutex::new(World::default())),
            schedule: Arc::new(Mutex::new(Schedule::default())),
            handle: None,
        }
    }
}
