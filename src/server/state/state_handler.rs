use bevy_ecs::{
    schedule::Schedule,
    world::{self, World},
};
use log::{debug, info};
use std::sync::{Arc, Mutex, RwLock};

use crate::server::{
    components::shared::{self, vec3d::Vec3d},
    packet_sender::packet_sender::{PacketSender, ServerPacketSender, ServerPacketSenderState},
    packets::{self, packet::Packet},
    systems::{
        self,
        command_container::{self, CommandContainer},
    },
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
    pub(super) sender: Arc<Mutex<ServerPacketSender>>,
}

impl ServerStateHandler {
    pub fn new(
        ticker: Arc<Mutex<dyn TickerTrait>>,
        sender: Arc<Mutex<ServerPacketSender>>,
    ) -> Self {
        let schedule = Schedule::default();

        ServerStateHandler {
            world: Arc::new(RwLock::new(World::default())),
            schedule: Arc::new(Mutex::new(schedule)),
            ticker,
            sender,
        }
    }
}

impl StateHandler for ServerStateHandler {
    fn start(&mut self) {
        let world = self.world.clone();
        let schedule = self.schedule.clone();

        info!("Starting server state handler");

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

        let shared_world = self.world.clone();
        let shared_sender = self.sender.clone();

        self.ticker.lock().unwrap().register(Box::new(move || {
            let mut world = shared_world.write().unwrap();

            let command_container = world
                .resource_mut::<CommandContainer<Vec3d>>()
                .entries
                .clone();

            debug!(
                "Enqueuing packets from {} commands",
                command_container.len()
            );

            // map commands to packets from world resources

            shared_sender.lock().unwrap().enqueue(Packet {
                id: 0,
                opcode: crate::server::opcode::OpCode::Spawn,
                data: "".to_string(),
            });
            debug!("Done enqueing packets");
        }));

        self.ticker.lock().unwrap().register(Box::new(move || {
            let mut world = world.write().unwrap();
            let mut schedule = schedule.lock().unwrap();

            debug!("Running schedule");
            let now = std::time::Instant::now();
            schedule.run(&mut world);
            debug!("Schedule run complete in: {:?}", now.elapsed().as_millis());
        }));

        self.ticker.lock().unwrap().run();
    }

    fn get_world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}
