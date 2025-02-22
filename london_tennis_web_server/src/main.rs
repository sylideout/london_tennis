use reqwest::Client;
use london_tennis_scraper::fetch_court_availabilities;


#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = Client::new();
    let _ = fetch_court_availabilities(client, None).await;
}
