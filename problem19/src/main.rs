use chrono::Datelike;
use chrono::{NaiveDate, Weekday};

fn main() {
    let mut sundays = 0;
    for year in 1901..=2000 {
        for month in 1..=12 {
            let date = NaiveDate::from_ymd_opt(year, month, 1);
            if let Some(date) = date {
                if date.weekday() == Weekday::Sun {
                    sundays += 1;
                }
            }
        }
    }
    println!("{}", sundays);
}
