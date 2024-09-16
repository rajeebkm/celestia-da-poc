use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::adapters::traits::AdapterFunctions;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
       .service(submit_blob);
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/submit_blob")]
async fn submit_blob(
    req_body: String,
    adapter: web::Data<Arc<Mutex<dyn AdapterFunctions>>>
) -> impl Responder {
    let adapter = adapter.lock().await;

    let result = adapter.push_data(b"namespace_id", req_body.as_bytes()).await;

    match result {
        Some(height) => HttpResponse::Ok().body(format!("Submitted at height: {:?}", height)),
        None => HttpResponse::InternalServerError().body("Failed to submit blob")
    }
}
