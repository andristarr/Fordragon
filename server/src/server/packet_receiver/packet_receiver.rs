use log::{debug, warn};

use crate::server::packet_handler::builder::PacketHandlerBuilder;
use crate::server::packet_handler::packet_handler::PacketHandlerTrait;
use crate::server::packets::packet::Packet;
use crate::server::state::state_handler::StateHandler;
use crate::server::state::ticker::TickerTrait;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub trait PacketReceiver: Send + Sync {
    fn consume(&self, packet: Packet, addr: SocketAddr);
    fn initialise(&mut self);
}

pub struct ServerPacketReceiver {
    ticker: Arc<Mutex<dyn TickerTrait>>,
    state: Arc<Mutex<ServerPacketReceiverState>>,
    packet_handler: Arc<Mutex<dyn PacketHandlerTrait>>,
}

pub struct ServerPacketReceiverState {
    pub(super) state_handler: Box<dyn StateHandler>,
    connections: HashMap<SocketAddr, u128>,
}

impl ServerPacketReceiver {
    pub fn new(state_handler: Box<dyn StateHandler>, ticker: Arc<Mutex<dyn TickerTrait>>) -> Self {
        let state = ServerPacketReceiverState {
            state_handler,
            connections: HashMap::new(),
        };

        let state = Arc::new(Mutex::new(state));

        let packet_handler = PacketHandlerBuilder::new()
            .with_enter_handler()
            .with_move_handler()
            .build();

        ServerPacketReceiver {
            ticker,
            state,
            packet_handler: Arc::new(Mutex::new(packet_handler)),
        }
    }

    pub fn inject_packets(
        packet_handler: Arc<Mutex<dyn PacketHandlerTrait>>,
        state: Arc<Mutex<ServerPacketReceiverState>>,
    ) {
        packet_handler
            .lock()
            .expect("Failed to lock packet handler")
            .transform_state(
                state
                    .lock()
                    .expect("Failed to lock packet receiver state")
                    .state_handler
                    .get_world(),
            );
    }
}

impl PacketReceiver for ServerPacketReceiver {
    fn consume(&self, packet: Packet, addr: SocketAddr) {
        let packet_id = packet.id;

        if self.state.lock().unwrap().connections.get(&addr).is_none() {
            self.state
                .lock()
                .unwrap()
                .connections
                .insert(addr, packet_id);
        } else if self.state.lock().unwrap().connections.get(&addr).unwrap() > &packet_id {
            warn!("Packet loss detected, dropping packet...");
            return;
        } else {
            self.state
                .lock()
                .unwrap()
                .connections
                .insert(addr, packet_id);
        }

        self.packet_handler
            .lock()
            .expect("Failed to lock packet handler")
            .handle_packet(addr, packet);
    }

    fn initialise(&mut self) {
        let state = self.state.clone();
        let packet_handler = self.packet_handler.clone();

        self.ticker.lock().unwrap().register(Box::new(move || {
            debug!("Injecting packets...");

            ServerPacketReceiver::inject_packets(packet_handler.clone(), state.clone());
        }));

        self.state.lock().unwrap().state_handler.start();
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::world::World;

    use super::*;

    use std::sync::{Arc, Mutex, RwLock};

    struct MockStateHandler {}

    impl StateHandler for MockStateHandler {
        fn start(&mut self) {}
        fn get_world(&self) -> Arc<RwLock<World>> {
            Arc::new(RwLock::new(World::default()))
        }
    }

    struct MockPacketHandler {
        pub called: Arc<Mutex<bool>>,
    }
    impl PacketHandlerTrait for MockPacketHandler {
        fn handle_packet(&mut self, _addr: std::net::SocketAddr, _packet: Packet) {}
        fn clear_packets(&mut self) {}
        fn transform_state(&mut self, _world: Arc<RwLock<World>>) {
            *self.called.lock().unwrap() = true;
        }
    }

    #[test]
    fn test_inject_packets_calls_transform_state() {
        let called = Arc::new(Mutex::new(false));
        let handler = MockPacketHandler {
            called: called.clone(),
        };
        let state_handler = MockStateHandler {};

        let packet_handler: Arc<Mutex<dyn PacketHandlerTrait>> = Arc::new(Mutex::new(handler));
        let state = ServerPacketReceiverState {
            state_handler: Box::new(state_handler),
            connections: HashMap::new(),
        };
        let state = Arc::new(Mutex::new(state));

        ServerPacketReceiver::inject_packets(packet_handler.clone(), state.clone());

        assert!(
            *called.lock().unwrap(),
            "transform_state should have been called"
        );
    }

    #[cfg(test)]
    mod consume_tests {
        use super::*;
        use bevy_ecs::world::World;
        use std::collections::HashMap;
        use std::net::{Ipv4Addr, SocketAddr};
        use std::sync::RwLock;
        use std::sync::{Arc, Mutex};

        struct MockStateHandler;
        impl StateHandler for MockStateHandler {
            fn start(&mut self) {}
            fn get_world(&self) -> Arc<RwLock<World>> {
                Arc::new(RwLock::new(World::default()))
            }
        }

        struct MockPacketHandler {
            pub called: Arc<Mutex<Option<Packet>>>,
        }
        impl PacketHandlerTrait for MockPacketHandler {
            fn clear_packets(&mut self) {}
            fn handle_packet(&mut self, _addr: SocketAddr, packet: Packet) {
                *self.called.lock().unwrap() = Some(packet);
            }
            fn transform_state(&mut self, _world: Arc<RwLock<World>>) {}
        }

        fn make_receiver_with_handler(
            handler: Arc<Mutex<dyn PacketHandlerTrait>>,
            connections: HashMap<SocketAddr, u128>,
        ) -> ServerPacketReceiver {
            let state_handler = Box::new(MockStateHandler);
            let state = ServerPacketReceiverState {
                state_handler,
                connections,
            };
            ServerPacketReceiver {
                ticker: Arc::new(Mutex::new(MockTicker)),
                state: Arc::new(Mutex::new(state)),
                packet_handler: handler,
            }
        }

        struct MockTicker;
        impl TickerTrait for MockTicker {
            fn register(&mut self, _f: Box<dyn Fn() + Send>) {}

            fn run(&mut self) {}
        }

        fn test_addr() -> SocketAddr {
            SocketAddr::from((Ipv4Addr::LOCALHOST, 12345))
        }

        #[test]
        fn test_inserts_new_connection() {
            let called = Arc::new(Mutex::new(None));
            let handler = Arc::new(Mutex::new(MockPacketHandler {
                called: called.clone(),
            }));
            let receiver = make_receiver_with_handler(handler.clone(), HashMap::new());

            let packet = Packet {
                id: 42,
                opcode: crate::server::opcode::OpCode::Spawn,
                data: "".to_string(),
            };
            let addr = test_addr();

            receiver.consume(packet.clone(), addr);

            // Should insert connection
            let state = receiver.state.lock().unwrap();
            assert_eq!(state.connections.get(&addr), Some(&42));
            // Should call handle_packet
            assert_eq!(*called.lock().unwrap(), Some(packet));
        }

        #[test]
        fn test_packet_loss_detected() {
            let called = Arc::new(Mutex::new(None));
            let handler = Arc::new(Mutex::new(MockPacketHandler {
                called: called.clone(),
            }));

            let mut connections = HashMap::new();
            connections.insert(test_addr(), 100);
            let receiver = make_receiver_with_handler(handler.clone(), connections);

            let packet = Packet {
                id: 50,
                opcode: crate::server::opcode::OpCode::Spawn,
                data: "".to_string(),
            };
            let addr = test_addr();

            receiver.consume(packet.clone(), addr);

            // Should NOT update connection
            let state = receiver.state.lock().unwrap();
            assert_eq!(state.connections.get(&addr), Some(&100));
            // Should NOT call handle_packet
            assert_eq!(*called.lock().unwrap(), None);
        }

        #[test]
        fn test_updates_existing_connection_with_higher_id() {
            let called = Arc::new(Mutex::new(None));
            let handler = Arc::new(Mutex::new(MockPacketHandler {
                called: called.clone(),
            }));

            let mut connections = HashMap::new();
            connections.insert(test_addr(), 10);
            let receiver = make_receiver_with_handler(handler.clone(), connections);

            let packet = Packet {
                id: 20,
                opcode: crate::server::opcode::OpCode::Spawn,
                data: "".to_string(),
            };
            let addr = test_addr();

            receiver.consume(packet.clone(), addr);

            // Should update connection
            let state = receiver.state.lock().unwrap();
            assert_eq!(state.connections.get(&addr), Some(&20));
            // Should call handle_packet
            assert_eq!(*called.lock().unwrap(), Some(packet));
        }
    }

    #[cfg(test)]
    mod initialise_tests {
        use super::*;
        use bevy_ecs::world::World;
        use std::collections::HashMap;
        use std::sync::RwLock;
        use std::sync::{Arc, Mutex};

        struct MockStateHandler {
            pub start_called: Arc<Mutex<bool>>,
        }
        impl StateHandler for MockStateHandler {
            fn start(&mut self) {
                *self.start_called.lock().unwrap() = true;
            }
            fn get_world(&self) -> Arc<RwLock<World>> {
                Arc::new(RwLock::new(World::default()))
            }
        }

        struct MockPacketHandler;
        impl PacketHandlerTrait for MockPacketHandler {
            fn clear_packets(&mut self) {}
            fn handle_packet(&mut self, _addr: SocketAddr, _packet: Packet) {}
            fn transform_state(&mut self, _world: Arc<RwLock<World>>) {}
        }

        struct MockTicker {
            pub registered: Arc<Mutex<bool>>,
        }
        impl TickerTrait for MockTicker {
            fn register(&mut self, _f: Box<dyn Fn() + Send>) {
                *self.registered.lock().unwrap() = true;
            }
            fn run(&mut self) {}
        }

        #[test]
        fn test_initialise_registers_inject_packets_and_starts_state_handler() {
            let registered = Arc::new(Mutex::new(false));
            let ticker = MockTicker {
                registered: registered.clone(),
            };

            let start_called = Arc::new(Mutex::new(false));
            let state_handler = Box::new(MockStateHandler {
                start_called: start_called.clone(),
            });

            let state = ServerPacketReceiverState {
                state_handler,
                connections: HashMap::new(),
            };

            let mut receiver = ServerPacketReceiver {
                ticker: Arc::new(Mutex::new(ticker)),
                state: Arc::new(Mutex::new(state)),
                packet_handler: Arc::new(Mutex::new(MockPacketHandler)),
            };

            receiver.initialise();

            // Check that register was called
            assert!(
                *registered.lock().unwrap(),
                "Ticker register should have been called"
            );
            // Check that state_handler.start() was called
            assert!(
                *start_called.lock().unwrap(),
                "StateHandler start should have been called"
            );
        }
    }
}
