use crate::git::get_blame_data::BlameData;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PreparedBlameData {
    pub date: DateTime<Utc>,
    pub summary: String,
    pub author: String,
    pub commit: String,
    pub email: String,
}

pub fn prepare_blame_data(data: BlameData) -> PreparedBlameData {
    let timestamp: i64 = data.author_time.parse::<i64>().unwrap() * 1000;

    let naive = DateTime::from_timestamp(timestamp / 1000, 0)
        .expect("Invalid timestamp")
        .naive_utc();

    let mut date = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);

    let timezone_offset_hours: i32 = data.author_tz[0..3].parse().unwrap();
    let timezone_offset_minutes: i32 = data.author_tz[3..5].parse().unwrap();
    let total_offset_minutes =
        timezone_offset_hours * 60 + timezone_offset_minutes;

    date = date + Duration::minutes(total_offset_minutes as i64);

    let email = data.author_mail.replace("<", "").replace(">", "");

    PreparedBlameData {
        email,
        commit: data.commit[0..7].to_string(),
        date,
        summary: data.summary,
        author: data.author,
    }
}
