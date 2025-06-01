use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use bevy_ecs::world::World;
use log::debug;

use crate::server::{opcode::OpCode, packets::packet::Packet};

pub trait PacketHandlerTrait: Send + Sync {
    fn handle_packet(&mut self, packet: Packet);
    fn transform_state(&mut self, world: Arc<RwLock<World>>);
    fn clear_packets(&mut self);
}

pub struct PacketHandler {
    pub(super) handlers: HashMap<OpCode, Box<dyn PacketHandlerTrait>>,
}

impl PacketHandler {
    pub fn new() -> Self {
        PacketHandler {
            handlers: HashMap::new(),
        }
    }

    pub fn handle_packet(&mut self, packet: Packet) {
        debug!("Handling packet: {:?}", packet.opcode);

        if let Some(handler) = self.handlers.get_mut(&packet.opcode) {
            handler.handle_packet(packet);
        } else {
            debug!(
                "No handler found for packet with opcode: {:?}",
                packet.opcode
            );
        }
    }
}

impl PacketHandlerTrait for PacketHandler {
    fn handle_packet(&mut self, packet: Packet) {
        self.handle_packet(packet);
    }

    fn transform_state(&mut self, world: Arc<RwLock<World>>) {
        for handler in self.handlers.values_mut() {
            handler.transform_state(world.clone());
        }

        self.clear_packets();
    }

    fn clear_packets(&mut self) {
        for handler in self.handlers.values_mut() {
            handler.clear_packets();
        }
    }
}
