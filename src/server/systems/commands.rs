use crate::common::model::Vec3d;

pub struct MoveCommand {
    pub entity_instance: String,
    pub vector: Vec3d,
}