use super::packet::Packet;

pub struct Consumer {}

impl Consumer {
    pub fn enqueue(&mut self, _packet: Packet) {
        // bevy ecs running
    }
}
