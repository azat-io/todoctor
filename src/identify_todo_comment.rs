use crate::todo_keywords::TODO_KEYWORDS;

pub fn identify_todo_comment(comment_text: &str) -> bool {
    TODO_KEYWORDS
        .iter()
        .any(|keyword| comment_text.trim().to_uppercase().starts_with(keyword))
}
