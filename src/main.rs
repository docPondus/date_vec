use chrono::{DateTime, Datelike, NaiveDate, Utc};

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    println!("Servus am {} um {}!", utc.date_naive(), utc.time());

    let mut kalender = Vec::new();

    let tage = vec![
        NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
        NaiveDate::from_ymd_opt(2024, 12, 2).unwrap(),
        NaiveDate::from_ymd_opt(2024, 12, 3).unwrap(),
        NaiveDate::from_ymd_opt(2024, 12, 4).unwrap(),
        NaiveDate::from_ymd_opt(2024, 12, 5).unwrap(),
    ];

    for datum in tage {
        let wochentag = datum.weekday();
        kalender.push((datum, wochentag));
    }

    for (datum, wochentag) in &kalender {
        println!("{}, {}", wochentag, datum.format("%d. %B %Y"));
    }
}
