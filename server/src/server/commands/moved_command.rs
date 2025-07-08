use std::sync::{Arc, Mutex, RwLock};

use bevy_ecs::world::World;
use log::{debug, trace};
use serde::{Deserialize, Serialize};

use crate::server::{
    commands::{MapableCommand, StateMappedCommand},
    components::{networked::Networked, position::Position, shared::vec3d::Vec3d},
    opcode::OpCode,
    packet_sender::{
        packet_sender::{PacketSender, ServerPacketSender},
        send_packet::SendPacket,
        TargetAddress,
    },
    protocols::send::moved_packet::MovedPacket,
    systems::command_container::CommandContainer,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovedCommand {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl MovedCommand {
    pub fn new(id: String, x: f64, y: f64, z: f64) -> Self {
        MovedCommand { id, x, y, z }
    }
}

impl MapableCommand for MovedCommand {
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

impl StateMappedCommand for MovedCommand {
    fn map(world: Arc<RwLock<World>>, sender: Arc<Mutex<ServerPacketSender>>) {
        let mut world = world.write().expect("Failed to get write lock to world");
        let sender = sender.lock().expect("Failed to lock sender");

        debug!(
            "Enqueuing packets from {:?} moved commands",
            world
                .resource_mut::<CommandContainer<MovedCommand>>()
                .entries
                .iter()
                .map(|(_, queue)| queue.len())
                .sum::<usize>()
        );

        let mut commands_to_process = Vec::new();
        {
            let mut command_container = world.resource_mut::<CommandContainer<MovedCommand>>();
            for (_key, queue) in command_container.entries.iter_mut() {
                let cmds = queue.drain(..).collect::<Vec<MovedCommand>>();
                for cmd in cmds {
                    commands_to_process.push(cmd);
                }
            }
        }

        for cmd in commands_to_process {
            let packet = cmd.map_to_packet(&mut world);

            trace!("Enqueuing packet: {:?}", packet);

            let packet_data =
                serde_json::to_string(&packet).expect("Failed to serialize MovedCommand");

            sender.enqueue(SendPacket::new(
                packet_data,
                OpCode::Moved,
                TargetAddress::Broadcast,
            ));
        }
    }
}
