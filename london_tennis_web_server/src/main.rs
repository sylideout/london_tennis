use reqwest::Client;
use london_tennis_scraper::fetch_data;


#[tokio::main]
async fn main() {
    let client = Client::new();
    let url = "https://tennistowerhamlets.com/book/courts/poplar-rec-ground/2025-02-15#book";
    fetch_data(&client, url).await;
}
