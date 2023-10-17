use super::packet::Packet;
use super::packet_handler::PacketHandler;

pub struct Dispatcher {
    thread_count: u8,
    handlers: Vec<PacketHandler>,
}

impl Dispatcher {
    pub fn new(threads: u8) -> Self {
        let mut handlers: Vec<PacketHandler> = vec![];

        for _ in 1..threads {
            handlers.push(PacketHandler::new());
        }

        Dispatcher {
            thread_count: threads,
            handlers,
        }
    }

    pub fn enqueue(&mut self, packet: Packet) {
        let current = self.handlers.iter().min().unwrap();

        current.consume(packet);
    }
}
