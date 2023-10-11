use std::net::SocketAddr;
use std::sync::Arc;
use crate::common::config::Config;
use tokio::{net::UdpSocket, sync::mpsc};
use crate::common::error::Error;

pub struct Server {

}

impl Server {
    async fn run(config: Config) -> Result<(), Error> {
        let sock = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>()?).await?;

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
            tx.send((buf[..len].to_vec(), addr)).await.unwrap();
        }
    }
}