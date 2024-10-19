pub const PRIMARY_TODO_KEYWORDS: [&str; 9] = [
    "TODO", "FIXME", "CHANGED", "XXX", "HACK", "BUG", "OPTIMIZE", "REFACTOR",
    "TEMP",
];

pub const SECONDARY_TODO_KEYWORDS: [&str; 13] = [
    "IDEA",
    "NOTE",
    "REVIEW",
    "NB",
    "QUESTION",
    "DEBUG",
    "KLUDGE",
    "COMPAT",
    "WARNING",
    "DANGER",
    "INFO",
    "DEPRECATED",
    "COMBAK",
];

pub fn identify_todo_comment(comment_text: &str) -> Option<String> {
    let trimmed_text = comment_text.trim();

    let words_with_separators: Vec<&str> =
        trimmed_text.split_whitespace().collect();

    for keyword in PRIMARY_TODO_KEYWORDS.iter() {
        for word in &words_with_separators {
            if word.to_uppercase().contains(&keyword.to_uppercase()) {
                return Some(keyword.to_string());
            }
        }
    }

    if let Some(first_word) = words_with_separators.first() {
        for keyword in SECONDARY_TODO_KEYWORDS.iter() {
            if first_word.to_uppercase() == keyword.to_uppercase() {
                return Some(keyword.to_string());
            }
        }
    }

    None
}
