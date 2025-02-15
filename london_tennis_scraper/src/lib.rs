use reqwest::Client;
use std::error::Error;

mod parse_tower_hamlets;
mod locations;

pub async fn fetch_data(
    client: &Client,
    url: &str
) -> Result<(), Box<dyn Error>> {
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    parse_tower_hamlets::get_availability(body);
    Ok(())
}

pub fn get_next_seven_days() -> () {}
