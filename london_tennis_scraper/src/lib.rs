use reqwest::Client;
use std::error::Error;
use tokio;

mod parse_tower_hamlets;
mod locations;

struct CourtAvailability {
    date: String,
    time: String,
    court: String,
    available_courts: i8,
    url: String,
}

pub async fn fetch_data(
    client: Client,
    _loc: Option<locations::Council>
) -> () {
    let mut set = tokio::task::JoinSet::new();
    let urls = parse_tower_hamlets::generate_urls();

    for url in urls {
        let client_clone = client.clone();
        set.spawn(async move {
            let _ = scrape_data(client_clone, url).await;
        });
    }

    while let Some(_) = set.join_next().await {}
}

async fn scrape_data(client: Client, url: String) -> Result<(), Box<dyn Error>> {
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    parse_tower_hamlets::get_availability(body);
    Ok(())
}