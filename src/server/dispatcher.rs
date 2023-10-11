use crate::common::error::Error;
use crate::server::packet::Packet;
use crate::server::packet_handler::PacketHandler;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

pub struct Dispatcher {
    threads: u8,
    pub state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    pub packets: VecDeque<Packet>,
    pub handlers: Vec<PacketHandler>,
}

impl Dispatcher {
    pub fn new(threads: u8) -> Self {
        let mut handlers: Vec<PacketHandler> = vec![];

        let packets = VecDeque::new();

        for _ in 1..threads {
            handlers.push(PacketHandler::new());
        }

        Dispatcher {
            threads,
            state: Arc::new(Mutex::new(SharedState { packets, handlers })),
        }
    }

    pub fn run(&mut self, state: Arc<Mutex<SharedState>>) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                if state.lock().unwrap().packets.len() > 0 {
                    let handlers = state.lock().unwrap().handlers;

                    let consumer = handlers.iter().min().unwrap();

                    let packet = state.lock().unwrap().packets.pop_front().unwrap();

                    consumer.consume(packet);
                }
            }
        })
    }

    pub fn consume(&mut self, packet: Packet) -> Result<(), Error> {
        self.state.lock().unwrap().packets.push_back(packet);

        Ok(())
    }
}
