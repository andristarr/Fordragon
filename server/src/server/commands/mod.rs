use std::sync::{Arc, Mutex, RwLock};

use bevy_ecs::world::World;

use crate::server::packet_sender::packet_sender::ServerPacketSender;

pub mod move_command;
pub mod moved_command;
pub mod spawn_command;

pub trait MapableCommand {
    type PacketType;

    fn map_to_packet(&self, world: &mut World) -> Self::PacketType;
}

pub trait StateMappedCommand {
    fn map(world: Arc<RwLock<World>>, sender: Arc<Mutex<ServerPacketSender>>);
}
