use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use server::server::{components::position::Position, packets::packet::Packet};

use crate::udp_plugin::UdpPlugin;

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

    let mut app = App::new()
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
    // cube
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
    //     Transform::from_xyz(0.0, 0.5, 0.0),
    // ));
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

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: ResMut<CommandContainer>,
    mut query: Query<(&mut Position), (With<Position>)>,
) {
    let mut move_command = MoveCommand::new(0.0, 0.0, 0.0);

    for (pos) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            move_command.y = pos.position.y + 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            move_command.y = pos.position.y - 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            move_command.x = pos.position.x - 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            move_command.x = pos.position.x + 1.0;
        }
    }

    commands.commands.push(move_command);
}

#[derive(Debug, Resource, Default)]
pub struct CommandContainer {
    pub commands: Vec<MoveCommand>,
}

#[derive(Debug, Clone)]
pub struct MoveCommand {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl MoveCommand {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        MoveCommand { x, y, z }
    }
}

#[derive(Component)]
pub struct Identifier {
    pub id: String,
}
