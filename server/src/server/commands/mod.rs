use bevy_ecs::world::World;

pub mod move_command;
pub mod moved_command;
pub mod spawn_command;

pub trait MapableCommand {
    type PacketType;

    fn map_to_packet(&self, world: &mut World) -> Self::PacketType;
}
