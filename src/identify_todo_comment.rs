use crate::todo_keywords::TODO_KEYWORDS;

pub fn identify_todo_comment(comment_text: &str) -> Option<String> {
    let trimmed_text = comment_text.trim().to_uppercase();
    for keyword in TODO_KEYWORDS.iter() {
        if trimmed_text.starts_with(keyword) {
            return Some(keyword.to_string());
        }
    }
    None
}
