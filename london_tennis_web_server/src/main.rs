use reqwest::Client;
use london_tennis_scraper::fetch_data;


#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = Client::new();
    let url = "https://tennistowerhamlets.com/book/courts/poplar-rec-ground/2025-02-16#book";
    let _ = fetch_data(&client, url, None).await;
}
