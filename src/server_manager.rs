use axum::Router;
use axum::routing::get;
use colored::Colorize;
use tokio::net::TcpListener;
use crate::responses::hello_world;

pub struct Server {
    address: & 'static str
}

impl Server {
    pub fn new(address: & 'static str) -> Self {
        Self {address}
    }
    
    pub async fn start(&self) {
        let listener = TcpListener::bind(&self.address).await;
        match listener { 
            Ok(a) => {
                println!("{}{}", "Server successfully started on: ".bright_green(), a.local_addr().unwrap());
                axum::serve(a, router()).await.unwrap();
            }
            Err(e) => {
                println!("{}{}", "Could not start server: ", e)
            }
        }
        
        fn router() -> Router {
            Router::new().route("/hello-world", get(hello_world))
        }
    }
}

