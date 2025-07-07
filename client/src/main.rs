use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use server::server::{
    commands::move_command::MoveCommand,
    components::{
        movement_state::MovementStateType, networked::Networked, position::Position,
        shared::vec3d::Vec3d,
    },
    packets::packet::Packet,
};

use crate::udp_plugin::{IsMoving, OwnedEntityId, UdpPlugin};

mod udp_plugin;

fn main() {
    let (received_packets_sender, received_packets_receiver) = std::sync::mpsc::channel::<Packet>();
    let (packets_to_send_sender, packets_to_send_receiver) = std::sync::mpsc::channel::<Packet>();

    let received_packets_sender = Arc::new(Mutex::new(received_packets_sender));
    let received_packets_receiver = Arc::new(Mutex::new(received_packets_receiver));
    let packets_to_send_sender = Arc::new(Mutex::new(packets_to_send_sender));
    let packets_to_send_receiver = Arc::new(Mutex::new(packets_to_send_receiver));

    let udp_plugin = UdpPlugin::new(
        received_packets_sender.clone(),
        received_packets_receiver.clone(),
        packets_to_send_sender.clone(),
        packets_to_send_receiver.clone(),
    );

    let _ = App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
        .insert_resource(CommandContainer::default())
        .add_plugins(udp_plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input,))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// handling combined input from keyboard
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: ResMut<CommandContainer>,
    owned_entity_id: Res<OwnedEntityId>,
    mut is_moving: ResMut<IsMoving>,
    mut query: Query<(&mut Position, &mut Networked), (With<Position>, With<Networked>)>,
) {
    let mut move_command =
        MoveCommand::new("".to_string(), 0.0, 0.0, 0.0, MovementStateType::Moving);

    let mut has_move = false;

    if keyboard_input.pressed(KeyCode::KeyW) {
        move_command.z = -1.0;
        has_move = true;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        move_command.z = 1.0;
        has_move = true;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        move_command.x = -1.0;
        has_move = true;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        move_command.x = 1.0;
        has_move = true;
    }

    if is_moving.0.x == move_command.x
        && is_moving.0.z == move_command.z
        && is_moving.0.y == move_command.y
    {
        return;
    }

    is_moving.0 = Vec3d::new(move_command.x, move_command.y, move_command.z);

    if !has_move {
        move_command.state = MovementStateType::Stopped;
    }

    println!("Sending move command: {:?}", move_command);

    commands.move_commands.push(move_command);
}

#[derive(Debug, Resource, Default)]
pub struct CommandContainer {
    pub move_commands: Vec<MoveCommand>,
}
