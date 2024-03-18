use bevy_ecs::{schedule::Schedule, world::World};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

use super::systems;

pub struct StateHandler {
    world: Arc<Mutex<World>>,
    schedule: Arc<Mutex<Schedule>>,
    handle: Option<JoinHandle<()>>,
}

impl StateHandler {
    pub fn new() -> StateHandler {
        StateHandler {
            world: Arc::new(Mutex::new(World::default())),
            schedule: Arc::new(Mutex::new(Schedule::default())),
            handle: None,
        }
    }

    pub fn run(&mut self) {
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

    pub fn get_world(&self) -> Arc<Mutex<World>> {
        self.world.clone()
    }
}
