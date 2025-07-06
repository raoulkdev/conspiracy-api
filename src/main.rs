use crate::server_manager::Server;

mod server_manager;
mod responses;

#[tokio::main]
async fn main() {
    let server = Server::new("127.0.0.1:3000");
    server.start().await;
}
