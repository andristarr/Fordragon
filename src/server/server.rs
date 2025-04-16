use crate::server::packet_receiver::packet_receiver::PacketReceiver;
use crate::server::packets::packet::Packet;
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{net::UdpSocket, sync::mpsc};

pub struct Server<T: PacketReceiver> {
    packet_handler: T,
}

impl<T: PacketReceiver> Server<T> {
    pub fn new(mut packet_handler: T) -> Self {
        packet_handler.initialise();

        Server { packet_handler }
    }

    pub async fn run(&mut self) -> Result<()> {
        let sock = UdpSocket::bind("0.0.0.0:1337".parse::<SocketAddr>()?).await?;

        println!("Server started on {:?}", sock.local_addr()?);

        let receiver = Arc::new(sock);
        let sender = receiver.clone();
        let (_tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

        tokio::spawn(async move {
            // sender
            while let Some((bytes, addr)) = rx.recv().await {
                let len = sender.send_to(&bytes, &addr).await.unwrap();
                println!("{:?} bytes sent", len);
            }
        });

        let mut buf = [0; 4096];

        let mut counter = 0;

        loop {
            // receiver
            let (len, addr) = receiver.recv_from(&mut buf).await?;
            // println!("[{:?}] {:?} bytes received from {:?}", counter, len, addr);

            let as_str = std::str::from_utf8(&buf[..len]).unwrap();

            let packet: Packet = serde_json::from_str::<Packet>(as_str).unwrap();

            self.packet_handler.consume(packet, addr);

            counter += 1;
        }
    }
}
