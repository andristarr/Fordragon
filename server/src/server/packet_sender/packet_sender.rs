use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use log::{debug, error, info, trace};
use tokio::{io::Interest, net::UdpSocket};

use crate::server::{
    packet_sender::{send_packet::SendPacket, TargetAddress},
    packets::packet::Packet,
    state::{packet_id_generator::PacketIdGenerator, ticker::TickerTrait},
};

pub struct ServerPacketSenderState {
    pub packet_datas: Vec<SendPacket>,
    pub connections: HashSet<SocketAddr>,
    pub socket: Option<Arc<UdpSocket>>,
}

// this is the trivial implementation where everything gets broadcasted to everyone
pub trait PacketSender: Send + Sync {
    fn try_register(&mut self, addr: SocketAddr);
    fn enqueue(&self, send_packet: SendPacket);
    fn initialise(&mut self, socket: Arc<UdpSocket>);
    fn emit_packets(
        packet_datas: Vec<SendPacket>,
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
            packet_datas: vec![],
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

    fn enqueue(&self, send_packet: SendPacket) {
        debug!(
            "Sending {:?} packet to {:?}",
            send_packet.opcode, send_packet.addr
        );

        let mut state = self.state.lock().unwrap();

        state.packet_datas.push(send_packet);
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

            let packets = std::mem::take(&mut state.packet_datas);

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
        packets: Vec<SendPacket>,
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

            let mut packets_by_addr: HashMap<SocketAddr, Vec<SendPacket>> = HashMap::new();

            for send_packet in packets.into_iter() {
                match &send_packet.addr {
                    TargetAddress::Broadcast => {
                        for addr in &connections {
                            packets_by_addr
                                .entry(*addr)
                                .or_default()
                                .push(send_packet.clone());
                        }
                    }
                    TargetAddress::Targeted(socket_addrs) => {
                        for addr in socket_addrs {
                            packets_by_addr
                                .entry(*addr)
                                .or_default()
                                .push(send_packet.clone());
                        }
                    }
                }
            }

            for addr in &connections {
                let packet_id_generator = packet_id_generator
                    .lock()
                    .expect("Failed to get lock to shared_id_generator");

                if let Some(send_packets) = packets_by_addr.get(addr) {
                    for send_packet in send_packets {
                        let packet = Packet {
                            id: packet_id_generator.generate_id(*addr),
                            opcode: send_packet.opcode,
                            data: send_packet.packet_data.clone(),
                        };

                        let bytes = match serde_json::to_vec(&packet) {
                            Ok(b) => b,
                            Err(e) => {
                                error!("Failed to serialize packet: {:?}", e);
                                continue;
                            }
                        };

                        let socket = socket.clone();
                        let addr = *addr;
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
            }

            join_all(send_futures).await;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::opcode::OpCode;
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

        {
            let state = sender.state.lock().unwrap();
            assert!(state.connections.contains(&addr));
            assert_eq!(state.connections.iter().filter(|&&a| a == addr).count(), 1);
        }
    }

    struct MockTicker;
    impl TickerTrait for MockTicker {
        fn register(&mut self, _f: Box<dyn Fn() + Send>) {}
        fn run(&mut self) {}
    }

    #[test]
    fn test_enqueue_adds_packet_to_state() {
        let ticker = Arc::new(Mutex::new(MockTicker));
        let packet_id_generator = Arc::new(Mutex::new(PacketIdGenerator::new()));
        let sender = ServerPacketSender::new(ticker, packet_id_generator);

        let data = "test".to_string();
        let opcode = OpCode::Spawn;
        let send_packet = SendPacket {
            addr: TargetAddress::Broadcast,
            opcode,
            packet_data: data.clone(),
        };

        sender.enqueue(send_packet);

        let state = sender.state.lock().unwrap();
        assert_eq!(state.packet_datas.len(), 1);
        assert_eq!(state.packet_datas[0].packet_data, data);
        assert_eq!(state.packet_datas[0].opcode, opcode);
    }

    #[test]
    fn test_enqueue_multiple_packets() {
        let ticker = Arc::new(Mutex::new(MockTicker));
        let packet_id_generator = Arc::new(Mutex::new(PacketIdGenerator::new()));
        let sender = ServerPacketSender::new(ticker, packet_id_generator);

        let data1 = "first".to_string();
        let data2 = "second".to_string();
        let opcode = OpCode::Spawn;
        let send_packet1 = SendPacket {
            addr: TargetAddress::Broadcast,
            opcode,
            packet_data: data1.clone(),
        };
        let send_packet2 = SendPacket {
            addr: TargetAddress::Broadcast,
            opcode,
            packet_data: data2.clone(),
        };

        sender.enqueue(send_packet1);
        sender.enqueue(send_packet2);

        let state = sender.state.lock().unwrap();
        assert_eq!(state.packet_datas.len(), 2);
        assert_eq!(state.packet_datas[0].packet_data, data1);
        assert_eq!(state.packet_datas[0].opcode, opcode);
        assert_eq!(state.packet_datas[1].packet_data, data2);
        assert_eq!(state.packet_datas[1].opcode, opcode);
    }
}
