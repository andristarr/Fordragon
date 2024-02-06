mod common;
mod server;

use common::config::Config;
use server::server::Server;

#[tokio::main]
async fn main() {
    let _ = Server::run(Config::get().unwrap()).await;
}
