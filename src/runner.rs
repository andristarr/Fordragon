mod common;
mod server;

use server::server::Server;

#[tokio::main]
async fn main() {
    let _ = Server::run().await;
}
