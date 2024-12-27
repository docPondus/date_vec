use chrono::{DateTime, Datelike, Duration, NaiveDate, Weekday, Utc};

fn main() {
    
    let utc: DateTime<Utc> = Utc::now();
    println!("Servus am {} um {}!", utc.date_naive(), utc.time());

    let start_date = NaiveDate::from_ymd_opt(2024, 12, 1).expect("Kein gültiges Datum!");
    let end_date = NaiveDate::from_ymd_opt(2024, 12, 14).expect("Kein gültiges Datum!");

    let mut calendar: Vec<(NaiveDate, Weekday)> = Vec::new();

    let mut date = start_date;
    while date <= end_date {
        let weekday = date.weekday();
        calendar.push((date, weekday));
        date += Duration::days(1);
    }

    for (date_of_interest, weekday) in &calendar {
        println!("{}, {}", weekday, date_of_interest.format("%d. %B %Y"));
    }
}
