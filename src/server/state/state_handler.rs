use bevy_ecs::{
    schedule::{IntoSystemConfigs, Schedule},
    world::{self, World},
};
use log::{debug, info, trace};
use std::sync::{Arc, Mutex, RwLock};

use crate::server::{
    commands::move_command::MoveCommand,
    components::shared::{self, vec3d::Vec3d},
    packet_sender::packet_sender::{PacketSender, ServerPacketSender, ServerPacketSenderState},
    packets::{self, move_packet::MovePacket, packet::Packet},
    systems::{
        self,
        command_container::{self, CommandContainer},
    },
};

use super::{packet_id_generator::PacketIdGenerator, ticker::TickerTrait};

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
            .insert_resource(CommandContainer::<MoveCommand> {
                entries: Default::default(),
            });

        // system registrations here for now, should be in their own schedules
        // no meaningful systems yet, this is just to stress test, it seems around 300k entities it starts to slow down for the targeted 8/s tickrate

        let trivival_move_system = systems::trivial_move::trivival_move_system;
        let movement_system = systems::movement::movement_system;

        schedule.lock().unwrap().add_systems((
            trivival_move_system,
            movement_system.before(trivival_move_system),
        ));

        let shared_world = self.world.clone();
        let shared_sender = self.sender.clone();

        self.ticker.lock().unwrap().register(Box::new(move || {
            let mut world = world.write().unwrap();
            let mut schedule = schedule.lock().unwrap();

            debug!("Running schedule");
            let now = std::time::Instant::now();
            schedule.run(&mut world);
            debug!("Schedule run complete in: {:?}", now.elapsed().as_millis());
        }));

        self.ticker.lock().unwrap().register(Box::new(move || {
            let mut world = shared_world
                .write()
                .expect("Failed to get write lock to world");

            // map commands to packets from world resources
            // TODO correctly map here
            debug!(
                "Enqueuing packets from {:?} move commands",
                world
                    .resource_mut::<CommandContainer<MoveCommand>>()
                    .entries
                    .len()
            );

            let sender = shared_sender.lock().expect("Failed to lock sender");

            for command in world
                .resource_mut::<CommandContainer<MoveCommand>>()
                .entries
                .iter_mut()
            {
                trace!("Processing command: {:?}", command);
                let cmds = command.1.drain(..).collect::<Vec<MoveCommand>>();

                for cmd in cmds {
                    let packet = Packet {
                        id: None,
                        opcode: crate::server::opcode::OpCode::Movement,
                        data: serde_json::to_string(&MovePacket::new(
                            cmd.entity,
                            Vec3d {
                                x: cmd.x,
                                y: cmd.y,
                                z: cmd.z,
                            },
                        ))
                        .expect("Failed to serialize MoveCommand"),
                    };

                    trace!("Enqueuing packet: {:?}", packet);

                    sender.enqueue(packet);
                }
            }

            world
                .resource_mut::<CommandContainer<MoveCommand>>()
                .entries
                .iter_mut()
                .for_each(|(_, queue)| {
                    queue.clear();
                });

            trace!("Done enqueing packets");
        }));

        self.ticker.lock().unwrap().run();
    }

    fn get_world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}
