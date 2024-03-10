use crate::{
    common::{config::Config, error::Error},
    server::packet::Packet,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{net::UdpSocket, sync::mpsc};

use super::{
    dispatcher::Dispatcher,
    state_handler::{StateHandler},
};

pub struct Server {}

impl Server {
    pub async fn run() -> Result<(), Error> {
        let _config = Config::get()?;

        let mut state_handler = StateHandler::new();

        state_handler.run();

        let sock = UdpSocket::bind("0.0.0.0:1337".parse::<SocketAddr>()?).await?;

        let receiver = Arc::new(sock);
        let sender = receiver.clone();
        let (_tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

        tokio::spawn(async move {
            while let Some((bytes, addr)) = rx.recv().await {
                let len = sender.send_to(&bytes, &addr).await.unwrap();
                println!("{:?} bytes sent", len);
            }
        });

        let mut buf = [0; 1024];

        loop {
            let (len, addr) = receiver.recv_from(&mut buf).await?;
            println!("{:?} bytes received from {:?}", len, addr);

            let packet =
                serde_json::from_str::<Packet>(std::str::from_utf8(&buf).unwrap()).unwrap();

            println!("Received: {:?}", packet);

            Dispatcher::consume(packet);
        }
    }
}
