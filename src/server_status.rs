// src/check_status.rs

use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = "http://127.0.0.1:8080/";

    // Send a GET request to the server
    let response = client.get(url).send().await?;
    
    // Print the status code
    println!("Server response status: {}", response.status());

    // Optionally, print the body of the response
    let body = response.text().await?;
    println!("Response: {}", body);

    Ok(())
}
