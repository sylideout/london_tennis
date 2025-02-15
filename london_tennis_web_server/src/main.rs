use reqwest::Client;
use london_tennis_scraper::fetch_data;


#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = Client::new();
    let _ = fetch_data(client, None).await;
}
