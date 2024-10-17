use crate::types::TodoHistory;
use chrono::{Datelike, Duration, NaiveDate};
use std::collections::HashSet;

pub fn add_missing_days(
    mut todo_history_data: Vec<TodoHistory>,
    months: u32,
    current_todo_count: usize,
) -> Vec<TodoHistory> {
    if todo_history_data.is_empty() {
        let current_date = chrono::Utc::now().naive_utc().date();
        let last_date = current_date
            .checked_add_signed(Duration::days(30 * months as i64))
            .unwrap_or(current_date);

        let mut current_date_iter = current_date;
        let mut updated_history = Vec::new();

        while current_date_iter <= last_date {
            updated_history.push(TodoHistory {
                date: current_date_iter.format("%Y-%m-%d").to_string(),
                count: current_todo_count,
            });
            current_date_iter = current_date_iter
                .checked_add_signed(Duration::days(1))
                .unwrap();
        }

        return updated_history;
    }

    let mut existing_dates = HashSet::new();

    for entry in &todo_history_data {
        let date = NaiveDate::parse_from_str(&entry.date, "%Y-%m-%d")
            .expect("Invalid date format in history");
        existing_dates.insert(date);
    }

    todo_history_data.sort_by(|a, b| a.date.cmp(&b.date));

    let first_date = NaiveDate::parse_from_str(
        &todo_history_data.first().unwrap().date,
        "%Y-%m-%d",
    )
    .expect("Invalid date format in history");

    let last_date = first_date
        .checked_add_signed(Duration::days(30 * months as i64))
        .unwrap_or(first_date);

    let mut current_date = first_date;
    let mut previous_commit_count = todo_history_data.first().unwrap().count;

    while current_date <= last_date {
        if !existing_dates.contains(&current_date) {
            todo_history_data.push(TodoHistory {
                date: current_date.format("%Y-%m-%d").to_string(),
                count: previous_commit_count,
            });
        } else {
            let matching_entry = todo_history_data.iter().find(|e| {
                let date =
                    NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").unwrap();
                date.year() == current_date.year()
                    && date.month() == current_date.month()
            });
            if let Some(entry) = matching_entry {
                previous_commit_count = entry.count;
            }
        }

        current_date =
            current_date.checked_add_signed(Duration::days(1)).unwrap();
    }

    todo_history_data.sort_by(|a, b| b.date.cmp(&a.date));

    todo_history_data
}
