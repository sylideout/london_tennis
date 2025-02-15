use reqwest::Client;
use std::error::Error;


pub async fn fetch_data(client: &Client, url: &str) -> Result<(), Box<dyn Error>> {
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    println!("{body}");
    Ok(())
}
