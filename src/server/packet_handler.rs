use crate::server::consumer::Consumer;
use crate::server::packet::Packet;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

pub struct PacketHandler {
    pub job: Option<JoinHandle<()>>,
    pub packets: Vec<Packet>,
}

impl PacketHandler {
    pub fn consume(&self, packet: Packet) {
        todo!()
    }

    pub fn send(&self, packet: Packet, consumer: Consumer) {
        todo!()
    }

    pub fn get_queue_size(&self) {
        todo!()
    }

    pub fn new() -> PacketHandler {
        let packets = Vec::new();

        let shared = Arc::new(Mutex::new(&packets));

        let job: JoinHandle<()> = tokio::spawn(async move {
            loop {
                let qwe = shared.lock().unwrap();
            }
        });

        PacketHandler {
            job: Some(job),
            packets,
        }
    }
}

impl Eq for PacketHandler {}

impl PartialEq<Self> for PacketHandler {
    fn eq(&self, other: &Self) -> bool {
        self.get_queue_size() == other.get_queue_size()
    }
}

impl PartialOrd<Self> for PacketHandler {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_queue_size().cmp(&other.get_queue_size()))
    }
}

impl Ord for PacketHandler {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_queue_size().cmp(&other.get_queue_size())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.get_queue_size().cmp(&other.get_queue_size()) {
            Ordering::Greater => self,
            _ => other,
        }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.get_queue_size().cmp(&other.get_queue_size()) {
            Ordering::Less => self,
            _ => other,
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}
