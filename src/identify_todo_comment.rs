use lazy_static::lazy_static;
use regex::Regex;

pub const PRIMARY_TODO_KEYWORDS: [&str; 8] = [
    "TODO", "FIXME", "XXX", "HACK", "BUG", "OPTIMIZE", "REFACTOR", "TEMP",
];

pub const SECONDARY_TODO_KEYWORDS: [&str; 14] = [
    "CHANGED",
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

lazy_static! {
    static ref PRIMARY_KEYWORD_REGEXES: Vec<Regex> = PRIMARY_TODO_KEYWORDS
        .iter()
        .map(|keyword| Regex::new(&format!(
            r"(?i)\b{}\b",
            regex::escape(keyword)
        ))
        .unwrap())
        .collect();
    static ref SECONDARY_KEYWORD_REGEX: Regex =
        Regex::new(r"(?i)^[^\w]*(\w+)([\s\p{P}]*)(:|[\p{P}\s]|$)").unwrap();
}

pub fn identify_todo_comment(comment_text: &str) -> Option<String> {
    let trimmed_text = comment_text.trim();

    for (i, re) in PRIMARY_KEYWORD_REGEXES.iter().enumerate() {
        if re.is_match(trimmed_text) {
            return Some(PRIMARY_TODO_KEYWORDS[i].to_string());
        }
    }

    if let Some(captures) = SECONDARY_KEYWORD_REGEX.captures(trimmed_text) {
        let first_word = captures.get(1).unwrap().as_str();

        for keyword in SECONDARY_TODO_KEYWORDS.iter() {
            if first_word.eq_ignore_ascii_case(keyword) {
                return Some(keyword.to_string());
            }
        }
    }

    None
}
