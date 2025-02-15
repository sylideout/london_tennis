use scraper::{Html, Selector};

pub(crate) fn get_availability(html: String) -> () {
    let document = Html::parse_document(&html);
    let selector = Selector::parse("tr").unwrap();

    for element in document.select(&selector) {
        println!("Next line");
        println!("{:?}", element.text().collect::<Vec<&str>>());
    }

    extract_time_and_availability(String::from("test"));
}

pub(crate) fn generate_next_seven_days_link() -> Vec<String> {
    todo!()
}

fn extract_time_and_availability(html: String) -> () {}