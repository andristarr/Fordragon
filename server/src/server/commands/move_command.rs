use bevy_ecs::world::World;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::server::{
    commands::{MapableCommand, StateMappedCommand},
    components::{
        movement_state::MovementStateType, networked::Networked, position::Position,
        shared::vec3d::Vec3d,
    },
    protocols::send::moved_packet::MovedPacket,
    systems::command_container::CommandContainer,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveCommand {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub state: MovementStateType,
}

impl MoveCommand {
    pub fn new(id: String, x: f64, y: f64, z: f64, state: MovementStateType) -> Self {
        MoveCommand { id, x, y, z, state }
    }
}

impl MapableCommand for MoveCommand {
    type PacketType = MovedPacket;

    fn map_to_packet(&self, world: &mut World) -> Self::PacketType {
        let current_position = world
            .query::<(&Position, &Networked)>()
            .iter(&world)
            .find(|(_, networked)| networked.id == self.id)
            .map(|(position, _)| position.position.clone())
            .expect(&format!("Entity with ID {} not found", self.id));

        MovedPacket {
            networked_id: self.id.clone(),
            vector: Vec3d::new(current_position.x, current_position.y, current_position.z),
        }
    }
}

impl StateMappedCommand for MoveCommand {
    fn map(
        world: std::sync::Arc<std::sync::RwLock<World>>,
        _sender: std::sync::Arc<
            std::sync::Mutex<crate::server::packet_sender::packet_sender::ServerPacketSender>,
        >,
    ) {
        let mut world = world.write().expect("Failed to get write lock to world");

        debug!(
            "Enqueuing packets from {:?} move commands",
            world
                .resource_mut::<CommandContainer<MoveCommand>>()
                .entries
                .iter()
                .map(|(_, queue)| queue.len())
                .sum::<usize>()
        );

        {
            let mut command_container = world.resource_mut::<CommandContainer<MoveCommand>>();
            for (_, queue) in command_container.entries.iter_mut() {
                _ = queue.drain(..).collect::<Vec<MoveCommand>>();
            }
        }
    }
}
