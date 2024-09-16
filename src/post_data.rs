mod routes;
mod adapters;
use actix_web::{web, App, HttpServer};
use adapters::config::{AdapterTypes, fetch_adapter};
use std::sync::Arc;
use reqwest::Client;
use serde_json::json;
use std::error::Error;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = "http://127.0.0.1:8080/submit_blob";


    // Define the parameters as byte vectors
    let namespace_id = b"supersol";  // Raw byte data for namespace_id
    let blob_data = b"send_data_to_celestia_devnet";  // Raw byte data for blob_data

    // Encode the length of namespace_id as a 4-byte prefix
    let namespace_id_length = (namespace_id.len() as u32).to_be_bytes();

    // Create a body by concatenating the length prefix, namespace_id, and blob_data
    let mut body = Vec::with_capacity(4 + namespace_id.len() + blob_data.len());
    body.extend_from_slice(&namespace_id_length);  // Append length of namespace_id
    body.extend_from_slice(namespace_id);          // Append namespace_id bytes
    body.extend_from_slice(blob_data);             // Append blob_data bytes

    // Send POST request with raw binary data
    let response = client.post(url)
        .body(body)  // Attach raw binary data
        .send()
        .await?;

    // Print the status code
    println!("Server response status: {}", response.status());

    // Optionally, print the body of the response
    let body = response.text().await?;
    println!("Response: {}", body);

    Ok(())
}


