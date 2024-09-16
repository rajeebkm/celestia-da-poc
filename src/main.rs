use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::sync::Arc;

mod config;
mod routes;
mod adapters;

use config::AdapterConfiguration;
use adapters::{AdapterTypes, get_adapter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env

    let adapter_type = AdapterTypes::Celestia; // Example: use Celestia

    let (adapter, _adapter_config) = match adapter_type {
        AdapterTypes::Celestia => {
            let config = AdapterConfiguration {
                rpc_node: std::env::var("RPC_NODE_CELESTIA").expect("RPC_NODE_CELESTIA not set"),
                auth_token: std::env::var("AUTH_TOKEN_CELESTIA").expect("AUTH_TOKEN_CELESTIA not set"),
            };
            (get_adapter(AdapterTypes::Celestia, &config).await, config) // Pass config by reference
        },
        AdapterTypes::EigenLayer => {
            let config = AdapterConfiguration {
                rpc_node: std::env::var("RPC_NODE_EIGENLAYER").expect("RPC_NODE_EIGENLAYER not set"),
                auth_token: std::env::var("AUTH_TOKEN_EIGENLAYER").expect("AUTH_TOKEN_EIGENLAYER not set"),
            };
            (get_adapter(AdapterTypes::EigenLayer, &config).await, config) // Pass config by reference
        },
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&adapter)))
            .configure(routes::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
