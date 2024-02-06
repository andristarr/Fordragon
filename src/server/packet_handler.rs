use super::consumer::Consumer;
use super::packet::Packet;
use std::cmp::Ordering;

pub struct PacketHandler {}

impl PacketHandler {
    pub fn send(&self, packet: Packet, consumer: Consumer) {
        todo!()
    }

    pub fn get_queue_size(&self) {
        todo!()
    }

    pub fn new() -> Self {
        PacketHandler {}
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
