// Module Declarations
mod server_manager;
mod responses;

#[tokio::main]
async fn main() {
    // Create new server on localhost:3000
    let server = server_manager::Server::new("0.0.0.0:3000");
    
    // Start the server
    server.start().await;
}
