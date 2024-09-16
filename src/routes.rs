use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::adapters::traits::AdapterFunctions;
use serde::Deserialize;
// use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer};
use crate::adapters::config::{AdapterTypes, fetch_adapter};

#[derive(Deserialize)]
struct BlobQuery {
    height: u64,
    namespace_id: String,
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
       .service(submit_blob);
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Actix Web Server is running!")
}

#[post("/submit_blob")]
async fn submit_blob(
    body: web::Bytes
) -> impl Responder {
    let adapter_type = AdapterTypes::Celestia; // Example: use Celestia
    let (adapter, _adapter_config) = match adapter_type {
        AdapterTypes::Celestia => {
            fetch_adapter(AdapterTypes::Celestia).await
        },
        AdapterTypes::EigenLayer => {
            fetch_adapter(AdapterTypes::EigenLayer).await
        },
    };

    // Ensure the body has enough data to read the length prefix
    if body.len() < 4 {
        return HttpResponse::BadRequest().body("Invalid request: Not enough data");
    }
    
    // Extract the length prefix (first 4 bytes) and determine the size of namespace_id
    let namespace_id_length = u32::from_be_bytes(body[0..4].try_into().expect("Invalid length prefix"));
    
    // Ensure the body has enough data to read both namespace_id and blob_data
    if body.len() < (4 + namespace_id_length as usize) {
        return HttpResponse::BadRequest().body("Invalid request: Not enough data for namespace_id");
    }
    
    // Extract namespace_id and blob_data
    let namespace_id = &body[4..(4 + namespace_id_length as usize)];  // Extract namespace_id
    let blob_data = &body[(4 + namespace_id_length as usize)..];   

    let adapter = adapter.lock().await;
    let result = adapter.push_data(namespace_id, blob_data).await;

    match result {
        Some(height) => HttpResponse::Ok().body(format!("Submitted at height: {:?}", height)),
        None => HttpResponse::InternalServerError().body("Failed to submit blob")
    }
}

#[get("/get_blob")]
async fn get_blob(
    query: web::Query<BlobQuery>,
    adapter: web::Data<Arc<Mutex<dyn AdapterFunctions>>>
) -> impl Responder {
    let adapter = adapter.lock().await;

    // Handle the result from the pull_data function correctly
    match adapter.pull_data(query.height, query.namespace_id.as_bytes()).await {
        Ok(blob_data) => {
            HttpResponse::Ok().body(format!("Data: {:?}", String::from_utf8(blob_data).unwrap()))
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to get blob: {}", err)),
    }
}

