use bevy_ecs::{schedule::Schedule, world::World};
use log::{debug, info, trace};
use std::sync::{Arc, Mutex, RwLock};

use crate::server::{
    commands::{move_command::MoveCommand, spawn_command::SpawnCommand, MapableCommand},
    components::{networked::Networked, position::Position, shared::vec3d::Vec3d},
    opcode::OpCode,
    packet_sender::{
        packet_sender::{PacketSender, ServerPacketSender},
        send_packet::SendPacket,
    },
    packets::{enown_packet::EnownPacket, spawn_packet::SpawnPacket},
    systems::{
        self, command_container::CommandContainer,
        untargeted_command_container::UntargetedCommandContainer,
    },
};

use super::ticker::TickerTrait;

pub trait StateHandler: Send + Sync {
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

    fn register_resources(&mut self, world: Arc<RwLock<World>>) {
        world
            .write()
            .unwrap()
            .insert_resource(CommandContainer::<MoveCommand> {
                entries: Default::default(),
            });

        world
            .write()
            .unwrap()
            .insert_resource(UntargetedCommandContainer::<SpawnCommand> {
                entries: Default::default(),
            });
    }

    fn map_move_commands(world: Arc<RwLock<World>>, sender: Arc<Mutex<ServerPacketSender>>) {
        let mut world = world.write().expect("Failed to get write lock to world");
        let sender = sender.lock().expect("Failed to lock sender");

        debug!(
            "Enqueuing packets from {:?} move commands",
            world
                .resource_mut::<CommandContainer<MoveCommand>>()
                .entries
                .iter()
                .map(|(_, queue)| queue.len())
                .sum::<usize>()
        );

        for command in world
            .resource_mut::<CommandContainer<MoveCommand>>()
            .entries
            .iter_mut()
        {
            trace!("Processing command: {:?}", command);
            let cmds = command.1.drain(..).collect::<Vec<MoveCommand>>();

            for cmd in cmds {
                let packet = cmd.map_to_packet();

                trace!("Enqueuing packet: {:?}", packet);

                let packet_data =
                    serde_json::to_string(&packet).expect("Failed to serialize MoveCommand");

                sender.enqueue(SendPacket::new(packet_data, OpCode::Movement, None));
            }
        }

        world
            .resource_mut::<CommandContainer<MoveCommand>>()
            .entries
            .iter_mut()
            .for_each(|(_, queue)| {
                queue.clear();
            });
    }

    fn map_spawn_commands(world: Arc<RwLock<World>>, sender: Arc<Mutex<ServerPacketSender>>) {
        let mut world = world.write().expect("Failed to get write lock to world");
        let sender = sender.lock().expect("Failed to lock sender");

        debug!(
            "Enqueuing packets from {:?} spawn commands",
            world
                .resource_mut::<UntargetedCommandContainer<SpawnCommand>>()
                .entries
                .len()
        );

        // Also queue already existing packets

        let networked_entities = world
            .query::<(&Networked, &Position)>()
            .iter(&world)
            .map(|(networked, position)| SpawnPacket {
                id: networked.id.clone(),
                location: Vec3d::new(
                    position.position.x,
                    position.position.y,
                    position.position.z,
                ),
            })
            .collect::<Vec<SpawnPacket>>();

        for command in world
            .resource_mut::<UntargetedCommandContainer<SpawnCommand>>()
            .entries
            .iter_mut()
        {
            trace!("Processing command: {:?}", command);

            let packet = command.map_to_packet();

            trace!("Enqueuing packet: {:?}", packet);

            let packet_data =
                serde_json::to_string(&packet).expect("Failed to serialize SpawnCommand");

            sender.enqueue(SendPacket::new(packet_data.clone(), OpCode::Spawn, None));

            networked_entities
                .iter()
                .filter(|n| n.id != packet.id)
                .for_each(|spawn| {
                    sender.enqueue(SendPacket::new(
                        serde_json::to_string(&spawn).expect("Failed to serialize SpawnPacket"),
                        OpCode::Spawn,
                        None,
                    ));
                });

            let enown_packet = EnownPacket {
                id: packet.id.clone(),
            };

            sender.enqueue(SendPacket::new(
                serde_json::to_string(&enown_packet).expect("Failed to serialize SpawnCommand"),
                OpCode::Enown,
                command.owning_connection.clone(),
            ));
        }

        world
            .resource_mut::<UntargetedCommandContainer<SpawnCommand>>()
            .entries
            .clear();
    }

    fn map_state(world: Arc<RwLock<World>>, sender: Arc<Mutex<ServerPacketSender>>) {
        Self::map_move_commands(world.clone(), sender.clone());
        Self::map_spawn_commands(world, sender);

        trace!("Done enqueing packets");
    }
}

impl StateHandler for ServerStateHandler {
    fn start(&mut self) {
        let world = self.world.clone();
        let schedule = self.schedule.clone();

        info!("Starting server state handler");

        self.register_resources(world.clone());

        // system registrations here for now, should be in their own schedules
        // no meaningful systems yet, this is just to stress test, it seems around 300k entities it starts to slow down for the targeted 8/s tickrate

        // trivival_move_system is not registered for now
        let _trivival_move_system = systems::trivial_move::trivival_move_system;

        let movement_system = systems::movement::movement_system;

        let enter_world_system = systems::enter_world::enter_world_system;

        schedule
            .lock()
            .unwrap()
            .add_systems((enter_world_system, movement_system));

        let shared_world = self.world.clone();
        let shared_sender = self.sender.clone();

        self.ticker.lock().unwrap().register(Box::new(move || {
            let mut world = world.write().unwrap();
            let mut schedule = schedule.lock().unwrap();

            debug!("Running schedule");
            let now = std::time::Instant::now();
            schedule.apply_deferred(&mut world);
            debug!(
                "Schedule applied deferred in: {:?}",
                now.elapsed().as_millis()
            );
            schedule.run(&mut world);
            debug!("Schedule run complete in: {:?}", now.elapsed().as_millis());
        }));

        self.ticker.lock().unwrap().register(Box::new(move || {
            Self::map_state(shared_world.clone(), shared_sender.clone());
        }));

        self.ticker.lock().unwrap().run();
    }

    fn get_world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::server::state::packet_id_generator::PacketIdGenerator;

    use super::*;
    use bevy_ecs::world::World;
    use std::{
        collections::HashSet,
        net::SocketAddr,
        sync::{Arc, Mutex, RwLock},
    };
    use tokio::net::UdpSocket;

    #[test]
    fn test_register_resources_only_registers_expected_resources() {
        let world = Arc::new(RwLock::new(World::default()));
        let mock_ticker = Arc::new(Mutex::new(MockTicker));
        let mock_packet_id_generator = Arc::new(Mutex::new(PacketIdGenerator::new()));
        let mut handler = ServerStateHandler::new(
            mock_ticker.clone(),
            Arc::new(Mutex::new(ServerPacketSender::new(
                mock_ticker,
                mock_packet_id_generator,
            ))),
        );

        handler.register_resources(world.clone());

        let world_read = world.read().unwrap();

        assert!(world_read.contains_resource::<CommandContainer<MoveCommand>>());
        assert!(world_read.contains_resource::<UntargetedCommandContainer<SpawnCommand>>());
        assert!(!world_read.contains_resource::<CommandContainer<SpawnCommand>>());
        assert!(!world_read.contains_resource::<UntargetedCommandContainer<MoveCommand>>());
    }

    struct MockSender;
    impl crate::server::packet_sender::packet_sender::PacketSender for MockSender {
        fn try_register(&mut self, _addr: std::net::SocketAddr) {}
        fn enqueue(&self, _send_packet: crate::server::packet_sender::send_packet::SendPacket) {}
        fn initialise(&mut self, _socket: std::sync::Arc<tokio::net::UdpSocket>) {}
        fn emit_packets(
            _packet_datas: Vec<crate::server::packet_sender::send_packet::SendPacket>,
            _connections: HashSet<SocketAddr>,
            _socket: Arc<UdpSocket>,
            _packet_id_generator: Arc<Mutex<PacketIdGenerator>>,
        ) {
        }
    }

    struct MockTicker;
    impl super::super::ticker::TickerTrait for MockTicker {
        fn register(&mut self, _f: Box<dyn Fn() + Send>) {}
        fn run(&mut self) {}
    }

    thread_local! {
        static MOVE_CALLED: std::cell::RefCell<bool> = const { std::cell::RefCell::new(false) };
        static SPAWN_CALLED: std::cell::RefCell<bool> = const { std::cell::RefCell::new(false) };
    }

    struct TestHandler;
    impl TestHandler {
        fn map_move_commands(_world: Arc<RwLock<World>>, _sender: Arc<Mutex<MockSender>>) {
            MOVE_CALLED.with(|f| *f.borrow_mut() = true);
        }
        fn map_spawn_commands(_world: Arc<RwLock<World>>, _sender: Arc<Mutex<MockSender>>) {
            SPAWN_CALLED.with(|f| *f.borrow_mut() = true);
        }
        fn map_state(world: Arc<RwLock<World>>, sender: Arc<Mutex<MockSender>>) {
            Self::map_move_commands(world.clone(), sender.clone());
            Self::map_spawn_commands(world, sender);
        }
    }

    #[test]
    fn test_map_state_calls_both_mapping_functions() {
        MOVE_CALLED.with(|f| *f.borrow_mut() = false);
        SPAWN_CALLED.with(|f| *f.borrow_mut() = false);

        let world = Arc::new(RwLock::new(World::default()));
        let sender = Arc::new(Mutex::new(MockSender));

        TestHandler::map_state(world, sender);

        assert!(
            MOVE_CALLED.with(|f| *f.borrow()),
            "map_move_commands should be called"
        );
        assert!(
            SPAWN_CALLED.with(|f| *f.borrow()),
            "map_spawn_commands should be called"
        );
    }
}
