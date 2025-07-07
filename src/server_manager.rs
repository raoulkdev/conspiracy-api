// Imports
use axum::Router;
use axum::routing::get;
use colored::Colorize;
use tokio::net::TcpListener;
use crate::responses::{all_categories, all_theories, theories_by_category, theory_by_id};

// Server struct
pub struct Server {
    address: & 'static str
}

// Server implementation
impl Server {
    // Create a new server instance
    pub fn new(address: & 'static str) -> Self {
        Self {address}
    }
    
    // Start server
    pub async fn start(&self) {
        // Bind TCP listener to address
        let listener = TcpListener::bind(&self.address).await;
        
        // Check if binding was successfull
        match listener { 
            Ok(a) => {
                println!("{}{}", "Server successfully started on: ".bright_green(), a.local_addr().unwrap());
                axum::serve(a, router()).await.unwrap();
            }
            Err(e) => {
                println!("{}{}", "Could not start server: ", e)
            }
        }
        
        // Endpoint router
        fn router() -> Router {
            Router::new()
                .route("/theories", get(all_theories))
                .route("/theories/{capture}", get(theory_by_id))
                .route("/theories/", get(theories_by_category))
                .route("/categories", get(all_categories))
        }
    }
}

