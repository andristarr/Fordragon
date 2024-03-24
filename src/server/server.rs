use crate::server::packet_handler::packet_handler::PacketHandler;
use crate::{
    common::{config::Config, error::Error},
    server::packets::packet::Packet,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{net::UdpSocket, sync::mpsc};

use super::state_handler::state_handler::StateHandler;

pub struct Server {
    state_handler: Box<dyn StateHandler>,
    packet_handler: Box<dyn PacketHandler>,
}

impl Server {
    pub fn new(state_handler: impl StateHandler, packet_handler: impl PacketHandler) -> Self {
        Server {
            state_handler: Box::new(state_handler),
            packet_handler: Box::new(packet_handler),
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let _config = Config::get()?;

        self.state_handler.run();

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
