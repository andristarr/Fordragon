use bevy_ecs::{schedule::Schedule, world::World};
use std::sync::{Arc, Mutex, RwLock};

use crate::server::{
    components::shared::vec3d::Vec3d,
    systems::{self, command_container::CommandContainer},
};

use super::ticker::TickerTrait;

pub trait StateHandler {
    fn start(&mut self);
    fn get_world(&self) -> Arc<RwLock<World>>;
}

pub struct ServerStateHandler {
    pub(super) world: Arc<RwLock<World>>,
    pub(super) schedule: Arc<Mutex<Schedule>>,
    pub(super) ticker: Arc<Mutex<dyn TickerTrait>>,
}

impl ServerStateHandler {
    pub fn new(ticker: Arc<Mutex<dyn TickerTrait>>) -> Self {
        let schedule = Schedule::default();

        ServerStateHandler {
            world: Arc::new(RwLock::new(World::default())),
            schedule: Arc::new(Mutex::new(schedule)),
            ticker,
        }
    }
}

impl StateHandler for ServerStateHandler {
    fn start(&mut self) {
        let world = self.world.clone();
        let schedule = self.schedule.clone();

        println!("Starting server state handler");

        world
            .write()
            .unwrap()
            .insert_resource(CommandContainer::<Vec3d> {
                entries: Default::default(),
            });

        // system registrations here for now, should be in their own schedules
        // no meaningful systems yet, this is just to stress test, it seems around 300k entities it starts to slow down for the targeted 8/s tickrate
        schedule
            .lock()
            .unwrap()
            .add_systems(systems::movement::movement_system)
            .add_systems(systems::trivial_move::trivival_move_system);

        self.ticker.lock().unwrap().register(Box::new(move || {
            let mut world = world.write().unwrap();
            let mut schedule = schedule.lock().unwrap();
            println!("Running schedule");
            let now = std::time::Instant::now();
            schedule.run(&mut world);
            println!("Schedule run complete in: {:?}", now.elapsed().as_millis());
        }));

        self.ticker.lock().unwrap().run();
    }

    fn get_world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}
