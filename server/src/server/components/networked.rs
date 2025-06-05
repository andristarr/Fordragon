use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Networked {
    pub id: String,
}
