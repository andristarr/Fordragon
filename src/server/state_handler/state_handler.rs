use bevy_ecs::{schedule::Schedule, world::World};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

use crate::server::systems;

pub trait StateHandler {
    fn run(&mut self);
    fn get_world(&self) -> Arc<Mutex<World>>;
}

pub struct ServerStateHandler {
    pub(super) world: Arc<Mutex<World>>,
    pub(super) schedule: Arc<Mutex<Schedule>>,
    pub(super) handle: Option<JoinHandle<()>>,
}

impl ServerStateHandler {
    pub fn new() -> Self {
        ServerStateHandler {
            world: Arc::new(Mutex::new(World::default())),
            schedule: Arc::new(Mutex::new(Schedule::default())),
            handle: None,
        }
    }
}

impl StateHandler for ServerStateHandler {
    fn run(&mut self) {
        let world = self.world.clone();
        let schedule = self.schedule.clone();

        // system registrations here for now, should be in their own schedules
        schedule
            .lock()
            .unwrap()
            .add_systems(systems::movement::movement_system);

        let handle = tokio::spawn(async move {
            loop {
                let mut world = world.lock().unwrap();
                let mut schedule = schedule.lock().unwrap();
                schedule.run(&mut world);
            }
        });

        self.handle = Some(handle);
    }

    fn get_world(&self) -> Arc<Mutex<World>> {
        self.world.clone()
    }
}
