use crate::types::TodoHistory;
use chrono::{Duration, NaiveDate};
use std::collections::HashSet;

pub fn add_missing_days(
    mut todo_history_data: Vec<TodoHistory>,
    months: u32,
    current_todo_count: usize,
) -> Vec<TodoHistory> {
    if todo_history_data.is_empty() {
        let current_date = chrono::Utc::now().naive_utc().date();
        let start_date = current_date
            .checked_sub_signed(Duration::days(30 * months as i64))
            .unwrap_or(current_date);

        let mut current_date_iter = start_date;
        let mut updated_history = Vec::new();

        while current_date_iter <= current_date {
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

    let last_date = chrono::Utc::now().naive_utc().date();
    let start_date = last_date
        .checked_sub_signed(Duration::days(30 * months as i64))
        .unwrap_or(last_date);

    let mut current_date = start_date;
    let mut previous_commit_count = current_todo_count;

    while current_date <= last_date {
        if !existing_dates.contains(&current_date) {
            todo_history_data.push(TodoHistory {
                date: current_date.format("%Y-%m-%d").to_string(),
                count: previous_commit_count,
            });
        } else {
            if let Some(entry) = todo_history_data.iter().find(|e| {
                let date =
                    NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").unwrap();
                date == current_date
            }) {
                previous_commit_count = entry.count;
            }
        }

        current_date =
            current_date.checked_add_signed(Duration::days(1)).unwrap();
    }

    todo_history_data
        .into_iter()
        .filter(|entry| {
            let entry_date = NaiveDate::parse_from_str(&entry.date, "%Y-%m-%d")
                .expect("Invalid date format in history");
            entry_date >= start_date && entry_date <= last_date
        })
        .collect()
}
