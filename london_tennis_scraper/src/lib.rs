use reqwest::Client;
use std::error::Error;

mod parse_tower_hamlets;
mod locations;

pub async fn fetch_data(
    client: &Client,
    _url: &str,
    _loc: Option<locations::Council>
) -> Result<(), Box<dyn Error>> {
    // let response = client.get(url).send().await?;
    // let body = response.text().await?;
    
    // ADD ASYNC SPAWNS HERE
    // FETCH URLS
    let urls = parse_tower_hamlets::generate_urls();

    for i in urls {
        let response = client.get(i).send().await?;
        let body = response.text().await?;
        parse_tower_hamlets::get_availability(body);
    }

    // parse_tower_hamlets::get_availability(body);
    Ok(())
}

pub fn get_next_seven_days() -> () {}
