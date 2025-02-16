use reqwest::Client;
use std::error::Error;
use tokio;

mod parse_tower_hamlets;
mod locations;

pub struct CourtAvailability {
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
    let mut court_availabilities: Vec<String> = vec![];

    for url in urls {
        let client_clone = client.clone();
        set.spawn(async move {
            scrape_data(client_clone, url).await.unwrap()
        });
    }

    while let Some(res) = set.join_next().await {
        if let Ok((url, html)) = res {
            parse_tower_hamlets::get_court_availability(url, html);
            court_availabilities.push("test".to_string());
        }
    }
}

async fn scrape_data(client: Client, url: String) -> Result<(String, String), Box<dyn Error>> {
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    Ok((url, body))
}