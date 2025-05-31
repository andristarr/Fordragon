use crate::server::packet_receiver::packet_receiver::PacketReceiver;
use crate::server::packets::packet::Packet;
use anyhow::Result;
use log::{debug, info};
use std::sync::Arc;
use std::{net::SocketAddr, sync::Mutex};
use tokio::net::UdpSocket;

use super::packet_sender::packet_sender::{PacketSender, ServerPacketSender};

pub struct Server {
    packet_receiver: Box<dyn PacketReceiver>,
    packet_sender: Arc<Mutex<ServerPacketSender>>,
}

impl Server {
    pub fn new(
        mut packet_receiver: Box<dyn PacketReceiver>,
        packet_sender: Arc<Mutex<ServerPacketSender>>,
    ) -> Self {
        packet_receiver.initialise();

        Server {
            packet_receiver,
            packet_sender,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let sock = UdpSocket::bind("0.0.0.0:1337".parse::<SocketAddr>()?).await?;

        info!("Server started on {:?}", sock.local_addr()?);

        let receiver = Arc::new(sock);
        let sender = receiver.clone();

        self.packet_sender
            .lock()
            .unwrap()
            .initialise(sender.clone());

        let mut buf = [0; 4096];

        loop {
            // receiver
            let rec = receiver.recv_from(&mut buf).await;

            if let Err(e) = rec {
                debug!("Error receiving packet: {:?}", e);
                continue;
            }

            let (len, addr) = rec?;

            let as_str = std::str::from_utf8(&buf[..len]).unwrap();

            let packet: Packet = serde_json::from_str::<Packet>(as_str).unwrap();

            self.packet_receiver.consume(packet, addr);
            self.packet_sender.lock().unwrap().try_register(addr);
        }
    }
}
