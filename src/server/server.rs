use crate::server::packet_handler::packet_handler::PacketHandler;
use crate::{
    common::{config::Config, error::Error},
    server::packets::packet::Packet,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{net::UdpSocket, sync::mpsc};

use super::state_handler::state_handler::StateHandler;

pub struct Server<'a> {
    packet_handler: Box<dyn PacketHandler + 'a>,
}

impl<'a> Server<'a> {
    pub fn new(packet_handler: Box<dyn PacketHandler + 'a>) -> Self {
        Server { packet_handler }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let _config = Config::get()?;

        let sock = UdpSocket::bind("0.0.0.0:1337".parse::<SocketAddr>()?).await?;

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

        let mut buf = [0; 1024];

        loop {
            // receiver
            let (len, addr) = receiver.recv_from(&mut buf).await?;
            println!("{:?} bytes received from {:?}", len, addr);

            let packet =
                serde_json::from_str::<Packet>(std::str::from_utf8(&buf).unwrap()).unwrap();

            println!("Received: {:?}", packet);

            self.packet_handler.consume(packet);
        }
    }
}
