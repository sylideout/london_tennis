use scraper::{Html, Selector};
use chrono::{Local, Duration};
use crate::CourtAvailability;

mod courts;

pub(crate) fn generate_urls() -> Vec<String> {
    let mut full_urls: Vec<String> = Vec::new();
    let today = Local::now().date_naive();

    courts::TOWER_HAMLET_URLS
        .into_iter()
        .for_each(
        |url| {
            for i in 0..courts::DAYS_AHEAD+1 {
                let date = today + Duration::days(i.into());
                full_urls.push(format!("{url}{date}#book"));
            }
        }
    );
    full_urls
}

pub(crate) fn get_court_availability(url: String, html: String) -> () {
    let document = Html::parse_document(&html);
    let selector = Selector::parse("tr").unwrap();

    for element in document.select(&selector) {
        println!("Next line");
        println!("{:?}", element.text().collect::<Vec<&str>>());
    }
}

fn extract_date(url: &str) -> String {
    todo!()
}

fn extract_court(url: &str) -> String {
    todo!()
}

fn extract_time(html: Vec<&str>) -> String {
    todo!()
}

fn extract_available_courts(html: Vec<&str>) -> String {
    todo!()
}