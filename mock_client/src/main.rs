use std::{net::SocketAddr, sync::Arc, thread, time::Duration};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::net::UdpSocket;

pub struct MockClient {}

impl MockClient {
    pub async fn run(&mut self) -> Result<()> {
        let sock = UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>()?).await?;

        println!("Mock client started on {:?}", sock.local_addr()?);

        let receiver = Arc::new(sock);
        let sender = receiver.clone();

        let mut packet_id = 0;

        tokio::spawn(async move {
            loop {
                thread::sleep(Duration::from_millis(1_000));

                // sender
                let spawn_packets: Vec<SpawnPacket> = std::iter::repeat_n(
                    SpawnPacket {
                        location: Vec3d {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        spawned_type: "player".to_string(),
                    },
                    10,
                )
                .collect();

                println!("Length of packets: {:?}", spawn_packets.len());

                let mut counter = 0;

                for packet in spawn_packets {
                    let packet = serde_json::to_string(&Packet {
                        id: Some(packet_id),
                        opcode: OpCode::Spawn,
                        data: serde_json::to_string(&packet).unwrap(),
                    })
                    .unwrap();

                    packet_id += 1;

                    sender
                        .send_to(packet.as_bytes(), "127.0.0.1:1337")
                        .await
                        .unwrap();

                    counter += 1;

                    println!("Sent packet {:?}", counter);
                }
            }
        });

        let mut buf = [0; 1024];

        loop {
            // receiver
            let (len, addr) = receiver.recv_from(&mut buf).await?;
            println!("{:?} bytes received from {:?}", len, addr);

            let packet =
                serde_json::from_str::<Packet>(std::str::from_utf8(&buf[..len]).unwrap()).unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let mut client = MockClient {};

    client.run().await.unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3d { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpawnPacket {
    pub spawned_type: String,
    pub location: Vec3d,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash, Default)]
pub enum OpCode {
    #[default]
    Unset,
    Movement,
    Spawn,
}
