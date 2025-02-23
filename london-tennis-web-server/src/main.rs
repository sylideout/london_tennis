// use reqwest::Client;
// use london_tennis_scraper::fetch_court_availabilities;


// #[tokio::main(flavor = "current_thread")]
// async fn main() {
//     let client = Client::new();
//     let court_availabilities = fetch_court_availabilities(client, None).await;
//     println!("{:?}", court_availabilities);
// }

use std::env;
use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let dist_path = env::var("WEB_CLIENT").unwrap();
    HttpServer::new(move || App::new().service(
        fs::Files::new("/", &dist_path).index_file("index.html")))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
