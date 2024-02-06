use crate::{
    common::{config::Config, error::Error},
    server::packet::Packet,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{net::UdpSocket, sync::mpsc};

use super::dispatcher::Dispatcher;

pub struct Server {}

impl Server {
    pub async fn run(config: Config) -> Result<(), Error> {
        let sock = UdpSocket::bind("0.0.0.0:1337".parse::<SocketAddr>()?).await?;

        let mut dispatcher = Dispatcher::new(4);

        let r = Arc::new(sock);
        let s = r.clone();
        let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

        tokio::spawn(async move {
            while let Some((bytes, addr)) = rx.recv().await {
                let len = s.send_to(&bytes, &addr).await.unwrap();
                println!("{:?} bytes sent", len);
            }
        });

        let mut buf = [0; 1024];

        loop {
            let (len, addr) = r.recv_from(&mut buf).await?;
            println!("{:?} bytes received from {:?}", len, addr);

            let packet =
                serde_json::from_str::<Packet>(std::str::from_utf8(&buf).unwrap()).unwrap();

            println!("Received: {:?}", packet);
        }
    }
}
