use std::collections::HashSet;

pub fn remove_duplicate_dates(
    mut history: Vec<(String, String)>,
) -> Vec<(String, String)> {
    let mut seen_dates = HashSet::new();
    history.retain(|(_, date)| {
        if seen_dates.contains(date) {
            false
        } else {
            seen_dates.insert(date.clone());
            true
        }
    });
    history
}
