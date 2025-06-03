use std::{
    net::SocketAddr,
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender},
    },
    time::Duration,
};

use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use server::server::{
    components::{position::Position, shared::vec3d::Vec3d},
    opcode::OpCode,
    packets::{move_packet::MovePacket, packet::Packet},
};
use tokio::net::UdpSocket;

use crate::CommandContainer;

#[derive(Resource)]
pub struct SocketPackets {
    pub received_packets_receiver: Arc<Mutex<Receiver<Packet>>>,
    pub received_packets_sender: Arc<Mutex<Sender<Packet>>>,
    pub packets_to_send_receiver: Arc<Mutex<Receiver<Packet>>>,
    pub packets_to_send_sender: Arc<Mutex<Sender<Packet>>>,
}

pub struct UdpPlugin {
    received_packets_receiver: Arc<Mutex<Receiver<Packet>>>,
    received_packets_sender: Arc<Mutex<Sender<Packet>>>,
    packets_to_send_receiver: Arc<Mutex<Receiver<Packet>>>,
    packets_to_send_sender: Arc<Mutex<Sender<Packet>>>,
}

impl UdpPlugin {
    pub fn new(
        received_packets_sender: Arc<Mutex<Sender<Packet>>>,
        received_packets_receiver: Arc<Mutex<Receiver<Packet>>>,
        packets_to_send_sender: Arc<Mutex<Sender<Packet>>>,
        packets_to_send_receiver: Arc<Mutex<Receiver<Packet>>>,
    ) -> Self {
        UdpPlugin {
            received_packets_receiver,
            received_packets_sender,
            packets_to_send_receiver,
            packets_to_send_sender,
        }
    }
}

pub fn udp_system(
    mut commands: Commands,
    mut query: Query<(&mut Position)>,
    socket_packets: Res<SocketPackets>,
    mut command_container: ResMut<CommandContainer>,
) {
    let received_packets_receiver = socket_packets.received_packets_receiver.lock().unwrap();

    while let Ok(packet) = received_packets_receiver.try_recv() {
        match packet.opcode {
            OpCode::Spawn => {
                let spawned_packet: server::server::packets::spawn_packet::SpawnPacket =
                    serde_json::from_str(&packet.data).unwrap();

                println!("Spawn packet received: {:?}", spawned_packet);

                commands.spawn(Position {
                    position: spawned_packet.location,
                });
            }
            OpCode::Movement => {
                let move_packet: server::server::packets::move_packet::MovePacket =
                    serde_json::from_str(&packet.data).unwrap();

                println!("Move packet received: {:?}", move_packet);

                for mut entity in query.iter_mut() {
                    entity.position = move_packet.vector.clone();
                }
            }
            _ => {}
        }
    }

    let packets_to_send_sender = socket_packets.packets_to_send_sender.lock().unwrap();

    for (move_command) in command_container.commands.iter_mut() {
        let packet = Packet {
            id: None,
            opcode: OpCode::Movement,
            data: serde_json::to_string(&MovePacket::new(
                Entity::from_raw(0),
                Vec3d::new(move_command.x, move_command.y, move_command.z),
            ))
            .unwrap(),
        };

        packets_to_send_sender.send(packet).unwrap();
    }

    command_container.commands.clear();
}

pub fn start_listen_connection(
    runtime: Res<TokioTasksRuntime>,
    socket_packets: ResMut<SocketPackets>,
) {
    let received_packets_sender = socket_packets.received_packets_sender.clone();
    let packets_to_send_receiver = socket_packets.packets_to_send_receiver.clone();

    runtime.spawn_background_task(move |_| async move {
        let sock = UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap())
            .await
            .unwrap();

        println!("Mock client started on {:?}", sock.local_addr().unwrap());

        let receiver = Arc::new(sock);
        let sender = receiver.clone();

        // Sender task
        let packets_to_send_receiver_clone = packets_to_send_receiver.clone();
        let sender_clone = sender.clone();
        tokio::spawn(async move {
            loop {
                let curr = {
                    let packets_to_send_receiver = packets_to_send_receiver_clone.lock().unwrap();
                    packets_to_send_receiver.try_recv()
                };

                if curr.is_err() {
                    println!("No packets to send, waiting...");
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    continue;
                } else {
                    let p = curr.unwrap();
                    let packet_str = serde_json::to_string(&p).unwrap();
                    let packet_bytes = packet_str.as_bytes();

                    sender_clone
                        .send_to(packet_bytes, "127.0.0.1:1337")
                        .await
                        .unwrap();
                }
            }
        });

        // Receiver task
        let received_packets_sender_clone = received_packets_sender.clone();
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                // receiver
                let (len, addr) = receiver.recv_from(&mut buf).await.unwrap();
                println!("{:?} bytes received from {:?}", len, addr);

                let packet =
                    serde_json::from_str::<Packet>(std::str::from_utf8(&buf[..len]).unwrap())
                        .unwrap();

                received_packets_sender_clone
                    .lock()
                    .unwrap()
                    .send(packet)
                    .unwrap();
            }
        });
    });
}

impl Plugin for UdpPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SocketPackets {
            received_packets_receiver: self.received_packets_receiver.clone(),
            received_packets_sender: self.received_packets_sender.clone(),
            packets_to_send_receiver: self.packets_to_send_receiver.clone(),
            packets_to_send_sender: self.packets_to_send_sender.clone(),
        })
        .add_systems(Startup, start_listen_connection)
        .add_systems(Update, udp_system);
    }
}
