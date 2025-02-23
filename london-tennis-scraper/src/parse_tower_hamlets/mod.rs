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

pub(crate) fn generate_court_availabilities(url: String, html: String) -> Vec<CourtAvailability> {
    let date = extract_date(&url);
    let court = extract_court(&url);
    
    let document = Html::parse_document(&html);
    let selector = Selector::parse("tr").unwrap();

    let mut court_availabilities = Vec::new();

    for element in document.select(&selector) {
        let court_information = element.text().collect::<Vec<&str>>();
        let time = extract_time(&court_information);
        let available_courts = extract_available_courts(&court_information);
        
        court_availabilities.push(
            CourtAvailability::new()
                .date(date.clone())
                .time(time.clone())
                .court(court.clone())
                .available_courts(available_courts)
                .url(url.clone())
        )
    }
    court_availabilities
}

fn extract_date(url: &str) -> String {
    let mut split_string: Vec<&str> = url.split("/").collect();
    split_string = split_string[split_string.len()-1].split("#").collect();
    split_string[0].to_string()
}

fn extract_court(url: &str) -> String {
    let split_string: Vec<&str> = url.split("/").collect();
    split_string[split_string.len()-2].to_string()
}

fn extract_time(html: &Vec<&str>) -> String {
    let chars = html[0].to_string().chars().collect::<Vec<char>>();
    let ampm = chars[chars.len()-2..].iter().collect::<String>();
    let hour = chars[..chars.len()-2].iter().collect::<String>();

    let hour_24 = match ampm.as_str() {
        "am" if hour == "12" => 0i16,
        "am" => hour.parse::<i16>().unwrap(),
        "pm" if hour == "12" => 12i16,
        "pm" => hour.parse::<i16>().unwrap() + 12,
        _ => 0i16
    };

    format!("{:04}", hour_24*100)
}

fn extract_available_courts(html: &Vec<&str>) -> i8 {
    let mut available_courts = 0i8;
    html[1..].into_iter().for_each(
        |content| {
            if *content == "£4" || *content == "£6"{
                available_courts += 1;
            }
        } 
    );
    available_courts
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_date() {
        assert_eq!(extract_date(
            "https://tennistowerhamlets.com/book/courts/poplar-rec-ground/2025-02-28#book"
        ), "2025-02-28");
        assert_eq!(extract_date(
            "https://tennistowerhamlets.com/book/courts/poplar-rec-ground/2020-01-20#book"
        ), "2020-01-20");
    }

    #[test]
    fn test_extract_court() {
        assert_eq!(extract_court(
            "https://tennistowerhamlets.com/book/courts/poplar-rec-ground/2025-02-28#book"
        ), "poplar-rec-ground");
        assert_eq!(extract_court(
            "https://tennistowerhamlets.com/book/courts/wapping-gardens/2020-01-20#book"
        ), "wapping-gardens");
    }

    #[test]
    fn test_extract_time() {
        assert_eq!(extract_time(&vec!["8am"]), "0800");
        assert_eq!(extract_time(&vec!["12am"]), "0000");
        assert_eq!(extract_time(&vec!["8pm"]), "2000");
        assert_eq!(extract_time(&vec!["12pm"]), "1200");
    }

    #[test]
    fn test_extract_available_courts() {
        assert_eq!(extract_available_courts(
            &vec![
                "11am",
                "Court 1 - ",
                "Reserved",
                "Group coaching ",
                "Court 2 - ",
                "Reserved",
                "Group coaching "
            ]), 0);
        assert_eq!(extract_available_courts(
            &vec![
                "11am",
                "Court 1 - ",
                "£4",
                "Court 2 - ",
                "£6"
            ]), 2);
        assert_eq!(extract_available_courts(
            &vec![
                "4pm",
                "Court 1 - ",
                "Reserved",
                "Group coaching ",
                "Court 2",
                "£6",
                "Court 3",
                "booked",
                "Court 4",
                "booked"]
        ), 1);
    }
}
