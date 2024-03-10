use std::sync::{Arc, Mutex};
use bevy_ecs::{schedule::Schedule, world::World};
use tokio::task::JoinHandle;

pub struct StateHandler {
    world: Arc<Mutex<World>>,
    schedule: Arc<Mutex<Schedule>>,
    handle: Option<JoinHandle<()>>
}

impl StateHandler {
    pub fn new() -> StateHandler {
        StateHandler {
            world: Arc::new(Mutex::new(World::default())),
            schedule: Arc::new(Mutex::new(Schedule::default())),
            handle: None
        }
    }

    pub fn run(&mut self) {
        let world = self.world.clone();
        let schedule = self.schedule.clone();
        let handle = tokio::spawn(async move {
            
            // sync to only run for ticktime
            loop {
                let mut world = world.lock().unwrap();
                let mut schedule = schedule.lock().unwrap();
                schedule.run(&mut world);
            }
        });

        self.handle = Some(handle);
    }
}
