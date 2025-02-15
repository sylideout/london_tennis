use scraper::{Html, Selector};
use chrono::{Local, Duration};

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
    let orginal_urls = vec![
        "https://tennistowerhamlets.com/book/courts/bethnal-green-gardens/".to_string(),
        "https://tennistowerhamlets.com/book/courts/king-edward-memorial-park/".to_string(),
        "https://tennistowerhamlets.com/book/courts/poplar-rec-ground/".to_string(),
        "https://tennistowerhamlets.com/book/courts/ropemakers-field/".to_string(),
        "https://tennistowerhamlets.com/book/courts/st-johns-park/".to_string(),
        "https://tennistowerhamlets.com/book/courts/victoria-park/".to_string(),
        "https://tennistowerhamlets.com/book/courts/wapping-gardens/".to_string(),
    ];

    let today = Local::now().date_naive();

    orginal_urls
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