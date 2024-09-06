use bevy_ecs::{schedule::Schedule, world::World};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

use crate::server::systems;

use super::ticker::TickerTrait;

pub trait StateHandler {
    fn start(&mut self);
    fn get_world(&self) -> Arc<Mutex<World>>;
}

pub struct ServerStateHandler {
    pub(super) world: Arc<Mutex<World>>,
    pub(super) schedule: Arc<Mutex<Schedule>>,
    pub(super) ticker: Arc<Mutex<dyn TickerTrait>>,
}

impl ServerStateHandler {
    pub fn new(ticker: Arc<Mutex<dyn TickerTrait>>) -> Self {
        ServerStateHandler {
            world: Arc::new(Mutex::new(World::default())),
            schedule: Arc::new(Mutex::new(Schedule::default())),
            ticker,
        }
    }
}

impl StateHandler for ServerStateHandler {
    fn start(&mut self) {
        let world = self.world.clone();
        let schedule = self.schedule.clone();

        // system registrations here for now, should be in their own schedules
        schedule
            .lock()
            .unwrap()
            .add_systems(systems::movement::movement_system);

        self.ticker.lock().unwrap().register(Box::new(move || {
            let mut world = world.lock().unwrap();
            let mut schedule = schedule.lock().unwrap();
            schedule.run(&mut world);
        }));

        self.ticker.lock().unwrap().run();
    }

    fn get_world(&self) -> Arc<Mutex<World>> {
        self.world.clone()
    }
}
