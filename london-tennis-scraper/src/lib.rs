use reqwest::Client;
use std::error::Error;
use tokio;

mod parse_tower_hamlets;
mod locations;

#[derive(Debug)]
pub struct CourtAvailability {
    date: String,
    time: String,
    court: String,
    available_courts: i8,
    url: String,
}

impl CourtAvailability {
    fn new() -> Self {
        Self {
            date: String::from("2000-00-00"),
            time: String::from("0000"),
            court: String::from(""),
            available_courts: 0,
            url: String::from(""),
        }
    }

    fn date(mut self, date: String) -> Self {
        self.date = date;
        self
    }

    fn time(mut self, time: String) -> Self {
        self.time = time;
        self
    }

    fn court(mut self, court: String) -> Self {
        self.court = court;
        self
    }

    fn available_courts(mut self, available_courts: i8) -> Self {
        self.available_courts = available_courts;
        self
    }

    fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }
}

pub async fn fetch_court_availabilities(
    client: Client,
    _loc: Option<locations::Council>
) -> Vec<CourtAvailability> {
    let mut set = tokio::task::JoinSet::new();
    let urls = parse_tower_hamlets::generate_urls();
    let mut court_availabilities: Vec<CourtAvailability> = vec![];

    for url in urls {
        let client_clone = client.clone();
        set.spawn(async move {
            scrape_data(client_clone, url).await.unwrap()
        });
    }

    while let Some(res) = set.join_next().await {
        if let Ok((url, html)) = res {
            court_availabilities.extend(
                parse_tower_hamlets::generate_court_availabilities(url, html)
            );
        }
    }
    court_availabilities
}

async fn scrape_data(client: Client, url: String) -> Result<(String, String), Box<dyn Error>> {
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    Ok((url, body))
}
