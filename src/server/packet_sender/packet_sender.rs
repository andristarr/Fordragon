use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use log::{debug, error, info, trace};
use tokio::{io::Interest, net::UdpSocket};

use crate::server::{
    packets::packet::Packet,
    state::{packet_id_generator::PacketIdGenerator, ticker::TickerTrait},
};

pub struct ServerPacketSenderState {
    pub packets: Vec<Packet>,
    pub connections: HashSet<SocketAddr>,
    pub socket: Option<Arc<UdpSocket>>,
}

// this is the trivial implementation where everything gets broadcasted to everyone
pub trait PacketSender: Send + Sync {
    fn try_register(&mut self, addr: SocketAddr);
    fn enqueue(&self, packet: Packet);
    fn initialise(&mut self, socket: Arc<UdpSocket>);
    fn emit_packets(
        packets: Vec<Packet>,
        connections: HashSet<SocketAddr>,
        socket: Arc<UdpSocket>,
        packet_id_generator: Arc<Mutex<PacketIdGenerator>>,
    );
}

pub struct ServerPacketSender {
    state: Arc<Mutex<ServerPacketSenderState>>,
    ticker: Arc<Mutex<dyn TickerTrait>>,
    packet_id_generator: Arc<Mutex<PacketIdGenerator>>,
}

impl ServerPacketSender {
    pub fn new(
        ticker: Arc<Mutex<dyn TickerTrait>>,
        packet_id_generator: Arc<Mutex<PacketIdGenerator>>,
    ) -> Self {
        let state = ServerPacketSenderState {
            packets: vec![],
            connections: HashSet::new(),
            socket: None,
        };

        let state = Arc::new(Mutex::new(state));

        ServerPacketSender {
            state,
            ticker,
            packet_id_generator,
        }
    }
}

impl PacketSender for ServerPacketSender {
    fn try_register(&mut self, addr: SocketAddr) {
        let mut state = self.state.lock().unwrap();

        if !state.connections.contains(&Arc::new(addr)) {
            info!("Registering address {:?}", addr);
            state.connections.insert(addr);
        }
    }

    fn enqueue(&self, packet: Packet) {
        trace!("Sending packet {:?}", packet);

        let mut state = self.state.lock().unwrap();
        state.packets.push(packet);
    }

    fn initialise(&mut self, socket: Arc<UdpSocket>) {
        info!("Initialising packet sender");

        let mut state = self.state.lock().unwrap();

        state.socket = Some(socket.clone());

        let state = self.state.clone();

        let packet_id_generator = self.packet_id_generator.clone();

        self.ticker.lock().unwrap().register(Box::new(move || {
            // Emit packets every tick
            let mut state = state.lock().expect("Failed to lock packet sender state");

            let packets = std::mem::take(&mut state.packets);

            ServerPacketSender::emit_packets(
                packets,
                state.connections.clone(),
                state
                    .socket
                    .clone()
                    .expect("Socket should be initialized before emitting packets"),
                packet_id_generator.clone(),
            );
        }));
    }

    fn emit_packets(
        mut packets: Vec<Packet>,
        connections: HashSet<SocketAddr>,
        socket: Arc<UdpSocket>,
        packet_id_generator: Arc<Mutex<PacketIdGenerator>>,
    ) {
        trace!(
            "Emitting {} packets to {} connections",
            packets.len(),
            connections.len()
        );

        tokio::spawn(async move {
            use futures::future::join_all;

            let mut send_futures = vec![];

            let total_packets_sent = packets.len() * connections.len();
            debug!("Total packets to send: {}", total_packets_sent);

            for packet in packets.iter_mut() {
                let bytes = match serde_json::to_vec(&packet) {
                    Ok(b) => b,
                    Err(e) => {
                        error!("Failed to serialize packet: {:?}", e);
                        continue;
                    }
                };

                for addr in &connections {
                    packet_id_generator
                        .lock()
                        .expect("Failed to get lock to shared_id_generator")
                        .generate_id(*addr);

                    let socket = socket.clone();
                    let addr = addr.clone();
                    let bytes = bytes.clone();

                    let fut = async move {
                        let ready = socket.ready(Interest::WRITABLE).await;

                        if ready.is_ok() {
                            if ready.unwrap().is_writable() {
                                match socket.try_send_to(&bytes, addr) {
                                    Ok(sent) => trace!("Sent {} bytes to {:?}", sent, addr),
                                    Err(e) => error!("Failed to send packet: {:?}", e),
                                }
                            } else {
                                error!("Socket not writable, packet not sent to {:?}", addr);
                            }
                        }
                    };

                    send_futures.push(fut);
                }
            }

            join_all(send_futures).await;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, SocketAddr};
    use std::sync::{Arc, Mutex};

    fn test_addr(port: u16) -> SocketAddr {
        SocketAddr::from((Ipv4Addr::LOCALHOST, port))
    }

    #[test]
    fn test_try_register_adds_new_address() {
        let ticker = Arc::new(Mutex::new(MockTicker));
        let packet_id_generator = Arc::new(Mutex::new(PacketIdGenerator::new()));
        let mut sender = ServerPacketSender::new(ticker, packet_id_generator);

        let addr = test_addr(12345);

        {
            let state = sender.state.lock().unwrap();
            assert!(!state.connections.contains(&addr));
        }

        sender.try_register(addr);

        // Address should be present after registration
        {
            let state = sender.state.lock().unwrap();
            assert!(state.connections.contains(&addr));
        }
    }

    #[test]
    fn test_try_register_does_not_duplicate_address() {
        let ticker = Arc::new(Mutex::new(MockTicker));
        let packet_id_generator = Arc::new(Mutex::new(PacketIdGenerator::new()));
        let mut sender = ServerPacketSender::new(ticker, packet_id_generator);

        let addr = test_addr(12345);

        sender.try_register(addr);
        sender.try_register(addr);

        // Address should only be present once
        {
            let state = sender.state.lock().unwrap();
            assert_eq!(state.connections.iter().filter(|&&a| a == addr).count(), 1);
        }
    }

    #[cfg(test)]
    mod enqueue_tests {
        use crate::server::opcode::OpCode;

        use super::*;
        use std::sync::{Arc, Mutex};

        #[test]
        fn test_enqueue_adds_packet_to_state() {
            let ticker = Arc::new(Mutex::new(super::tests::MockTicker));
            let packet_id_generator = Arc::new(Mutex::new(PacketIdGenerator::new()));
            let sender = ServerPacketSender::new(ticker, packet_id_generator);

            let packet = Packet {
                id: Some(1),
                opcode: OpCode::Spawn,
                data: "test".to_string(),
            };

            sender.enqueue(packet.clone());

            let state = sender.state.lock().unwrap();
            assert_eq!(state.packets.len(), 1);
            assert_eq!(state.packets[0], packet);
        }

        #[test]
        fn test_enqueue_multiple_packets() {
            let ticker = Arc::new(Mutex::new(super::tests::MockTicker));
            let packet_id_generator = Arc::new(Mutex::new(PacketIdGenerator::new()));
            let sender = ServerPacketSender::new(ticker, packet_id_generator);

            let packet1 = Packet {
                id: Some(1),
                opcode: OpCode::Spawn,
                data: "first".to_string(),
            };
            let packet2 = Packet {
                id: Some(2),
                opcode: OpCode::Spawn,
                data: "second".to_string(),
            };

            sender.enqueue(packet1.clone());
            sender.enqueue(packet2.clone());

            let state = sender.state.lock().unwrap();
            assert_eq!(state.packets.len(), 2);
            assert_eq!(state.packets[0], packet1);
            assert_eq!(state.packets[1], packet2);
        }
    }

    struct MockTicker;
    impl TickerTrait for MockTicker {
        fn register(&mut self, _f: Box<dyn Fn() + Send>) {}
        fn run(&mut self) {}
    }
}
