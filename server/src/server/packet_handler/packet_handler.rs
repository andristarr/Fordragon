use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use bevy_ecs::world::World;
use log::{debug, trace};

use crate::server::{opcode::OpCode, packets::packet::Packet};

pub trait PacketHandlerTrait: Send + Sync {
    fn handle_packet(&mut self, addr: SocketAddr, packet: Packet);
    fn transform_state(&mut self, world: Arc<RwLock<World>>);
    fn clear_packets(&mut self);
}

pub struct PacketHandler {
    pub(super) handlers: HashMap<OpCode, Box<dyn PacketHandlerTrait>>,
}

impl Default for PacketHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketHandler {
    pub fn new() -> Self {
        PacketHandler {
            handlers: HashMap::new(),
        }
    }

    pub fn handle_packet(&mut self, addr: SocketAddr, packet: Packet) {
        debug!("Handling packet: {:?}", packet.opcode);

        trace!("Packet data: {:?}", packet.data);

        if let Some(handler) = self.handlers.get_mut(&packet.opcode) {
            handler.handle_packet(addr, packet);
        } else {
            debug!(
                "No handler found for packet with opcode: {:?}",
                packet.opcode
            );
        }
    }
}

impl PacketHandlerTrait for PacketHandler {
    fn handle_packet(&mut self, addr: SocketAddr, packet: Packet) {
        self.handle_packet(addr, packet);
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
