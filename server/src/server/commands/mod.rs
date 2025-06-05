pub mod move_command;
pub mod spawn_command;

pub trait MapableCommand {
    type PacketType;

    fn map_to_packet(&self) -> Self::PacketType;
}
