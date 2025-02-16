use std::collections::HashMap;
use scraper::{Html, Selector};
use chrono::{Local, Duration};

mod courts;

pub(crate) fn get_availability(html: String) -> () {
    let document = Html::parse_document(&html);
    let selector = Selector::parse("tr").unwrap();

    for element in document.select(&selector) {
        println!("Next line");
        println!("{:?}", element.text().collect::<Vec<&str>>());
    }
}

pub(crate) fn generate_urls() -> Vec<String> {
    let mut full_urls: Vec<String> = Vec::new();
    let today = Local::now().date_naive();

    courts::TOWER_HAMLET_URLS
        .into_iter()
        .for_each(
        |url| {
            for i in 0..8 {
                let date = today + Duration::days(i);
                full_urls.push(format!("{url}{date}#book"));
            }
        }
    );
    full_urls
}  