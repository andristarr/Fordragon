use crate::server::packet_receiver::packet_receiver::PacketReceiver;
use crate::server::packets::packet::Packet;
use anyhow::Result;
use log::{debug, info};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{net::UdpSocket, sync::mpsc};

use super::packet_sender::packet_sender::PacketSender;

pub struct Server<T: PacketReceiver, U: PacketSender> {
    packet_receiver: T,
    packet_sender: U,
}

impl<T: PacketReceiver, U: PacketSender> Server<T, U> {
    pub fn new(mut packet_receiver: T, mut packet_sender: U) -> Self {
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
        let (_tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

        self.packet_sender.initialise(sender.clone());

        tokio::spawn(async move {
            // sender
            while let Some((bytes, addr)) = rx.recv().await {
                let len = sender.send_to(&bytes, &addr).await.unwrap();
                debug!("{:?} bytes sent", len);
            }
        });

        let mut buf = [0; 4096];

        let mut counter = 0;

        loop {
            // receiver
            let (len, addr) = receiver.recv_from(&mut buf).await?;

            let as_str = std::str::from_utf8(&buf[..len]).unwrap();

            let packet: Packet = serde_json::from_str::<Packet>(as_str).unwrap();

            self.packet_receiver.consume(packet, addr);
            self.packet_sender.try_register(addr);

            counter += 1;
        }
    }
}
