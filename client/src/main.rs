use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
    vec,
};

use bevy::{
    DefaultPlugins,
    app::{App, RunFixedMainLoop, RunFixedMainLoopSystem, Startup},
    asset::Assets,
    color::Color,
    core_pipeline::core_3d::Camera3d,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        resource::Resource,
        schedule::{IntoScheduleConfigs, Schedule},
        system::{Commands, Query, Res, ResMut},
        world::World,
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::{
        Quat, Vec3,
        primitives::{Circle, Cuboid},
    },
    pbr::{MeshMaterial3d, PointLight, StandardMaterial},
    render::mesh::{Mesh, Mesh3d},
    transform::components::Transform,
    utils::default,
};
use serde::{Deserialize, Serialize};
use tokio::net::UdpSocket;

async fn start_listen_connection(
    emitted_packets: std::sync::mpsc::Receiver<Vec<Packet>>,
    received_packets: std::sync::mpsc::Sender<Packet>,
) {
    let sock = UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap())
        .await
        .unwrap();

    println!("Mock client started on {:?}", sock.local_addr().unwrap());

    let receiver = Arc::new(sock);
    let sender = receiver.clone();

    tokio::spawn(async move {
        loop {
            if let Ok(packets) = emitted_packets.recv() {
                for p in packets {
                    let packet_str = serde_json::to_string(&p).unwrap();
                    let packet_bytes = packet_str.as_bytes();

                    sender
                        .send_to(packet_bytes, "127.0.0.1:1337")
                        .await
                        .unwrap();
                }
            }
        }
    });

    tokio::spawn(async move {
        let mut buf = [0; 1024];

        loop {
            // receiver
            let (len, addr) = receiver.recv_from(&mut buf).await.unwrap();
            println!("{:?} bytes received from {:?}", len, addr);

            let packet =
                serde_json::from_str::<Packet>(std::str::from_utf8(&buf[..len]).unwrap()).unwrap();

            received_packets.send(packet).unwrap();
        }
    });
}

fn main() {
    let mut app = App::new();

    let (emitted_packets, received_packets) = std::sync::mpsc::channel::<Vec<Packet>>();
    let (received_packets_sender, received_packets_receiver) = std::sync::mpsc::channel::<Packet>();

    tokio::spawn(start_listen_connection(
        received_packets,
        received_packets_sender,
    ));

    let mut world = World::default();

    let mut world = Arc::new(Mutex::new(world));

    world
        .lock()
        .unwrap()
        .insert_resource(CommandContainer { commands: vec![] });

    tokio::spawn(async move {
        loop {
            if let Ok(packet) = received_packets_receiver.recv() {
                match packet.opcode {
                    OpCode::Movement => {
                        let move_packet: MovePacket = serde_json::from_str(&packet.data).unwrap();
                        handle_packets(vec![move_packet]);
                    }
                    OpCode::Spawn => {
                        // Handle spawn packets here
                    }
                    _ => {}
                }
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    let cloned_world = world.clone();

    tokio::spawn(async move {
        let mut packet_id = 0;
        loop {
            let commands = cloned_world
                .lock()
                .unwrap()
                .resource::<CommandContainer>()
                .commands
                .clone();

            if !commands.is_empty() {
                let mut move_commands = vec![];
                for command in &commands {
                    let move_packet = MovePacket::new(
                        Entity::from_raw(packet_id as u32),
                        Vec3d {
                            x: command.x,
                            y: command.y,
                            z: command.z,
                        },
                    );

                    let packet = Packet::new(
                        packet_id,
                        OpCode::Movement,
                        serde_json::to_string(&move_packet).unwrap(),
                    );

                    move_commands.push(packet);
                    packet_id += 1;
                }

                emitted_packets.send(move_commands).unwrap();
                cloned_world
                    .lock()
                    .unwrap()
                    .resource_mut::<CommandContainer>()
                    .commands
                    .clear();
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    let mut setup_system = Schedule::default();
    setup_system.add_systems(setup);

    setup_system.run(&mut world.lock().unwrap());

    let mut schedule = Schedule::default();
    let schedule =
        schedule.add_systems(handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop));

    loop {
        schedule.run(&mut world.lock().unwrap());
    }
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
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
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

fn handle_packets(mut packets: Vec<MovePacket>) {
    // let mut move_command = MoveCommand::new(0.0, 0.0, 0.0);

    // for (pos, _) in query.iter_mut() {
    //     if keyboard_input.pressed(KeyCode::KeyW) {
    //         move_command.y = pos.position.y + 1.0;
    //     }
    //     if keyboard_input.pressed(KeyCode::KeyS) {
    //         move_command.y = pos.position.y - 1.0;
    //     }
    //     if keyboard_input.pressed(KeyCode::KeyA) {
    //         move_command.x = pos.position.x - 1.0;
    //     }
    //     if keyboard_input.pressed(KeyCode::KeyD) {
    //         move_command.x = pos.position.x + 1.0;
    //     }
    // }

    // commands.commands.push(move_command);
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: ResMut<CommandContainer>,
    mut query: Query<(&mut Position, &Identifier), (With<Position>, With<Identifier>)>,
) {
    let mut move_command = MoveCommand::new(0.0, 0.0, 0.0);

    for (pos, _) in query.iter_mut() {
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

#[derive(Debug, Resource)]
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
pub struct Position {
    pub position: Vec3d,
}

#[derive(Component)]
pub struct Identifier {
    pub id: String,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovePacket {
    pub entity: Entity,
    pub vector: Vec3d,
}

impl MovePacket {
    pub fn new(entity: Entity, vector: Vec3d) -> Self {
        MovePacket { entity, vector }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpawnPacket {
    pub spawned_type: String,
    pub location: Vec3d,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Packet {
    pub id: Option<u128>,
    pub opcode: OpCode,
    pub data: String,
}

impl Packet {
    pub fn new(id: u128, opcode: OpCode, data: String) -> Self {
        Packet {
            id: Some(id),
            opcode,
            data,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash, Default)]
pub enum OpCode {
    #[default]
    Unset,
    Movement,
    Spawn,
}
