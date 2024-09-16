mod routes;
mod adapters;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use adapters::config::{AdapterTypes, fetch_adapter};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let adapter_type = AdapterTypes::Celestia; // Example: use Celestia

    let (adapter, _adapter_config) = match adapter_type {
        AdapterTypes::Celestia => {
            fetch_adapter(AdapterTypes::Celestia).await
        },
        AdapterTypes::EigenLayer => {
            fetch_adapter(AdapterTypes::EigenLayer).await
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


