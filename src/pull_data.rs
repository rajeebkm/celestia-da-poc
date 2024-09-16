use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let base_url = "http://127.0.0.1:8080/get_blob";

    // Define the query parameters
    let height = 12345;
    let namespace_id = "rkm";

    // Build the URL with query parameters
    let url = format!("{}?height={}&namespace_id={}", base_url, height, namespace_id);

    // Send a GET request with the query parameters
    let response = client.get(&url).send().await?;

    // Print the status code
    println!("Server response status: {}", response.status());

    // Optionally, print the body of the response
    let body = response.text().await?;
    println!("Response: {}", body);

    Ok(())
}
