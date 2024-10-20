use todoctor::identify_todo_comment::identify_todo_comment;

#[tokio::test]
async fn test_primary_keyword_found() {
    let comment = "// TODO: This is a test comment.";
    assert_eq!(identify_todo_comment(comment), Some("TODO".to_string()));
}

#[tokio::test]
async fn test_primary_keyword_middle_of_sentence() {
    let comment = "// This is a test TODO comment.";
    assert_eq!(identify_todo_comment(comment), Some("TODO".to_string()));
}

#[tokio::test]
async fn test_primary_keyword_at_end() {
    let comment = "// This is a test TODO";
    assert_eq!(identify_todo_comment(comment), Some("TODO".to_string()));
}

#[tokio::test]
async fn test_primary_keyword_lowercase() {
    let comment = "// todo: lowercase todo comment.";
    assert_eq!(identify_todo_comment(comment), Some("TODO".to_string()));
}

#[tokio::test]
async fn test_secondary_keyword_at_start_with_colon() {
    let comment = "// NB: There is a nota bene.";
    assert_eq!(identify_todo_comment(comment), Some("NB".to_string()));
}

#[tokio::test]
async fn test_secondary_keyword_at_start_with_no_colon() {
    let comment = "// DEBUG: There is a debug.";
    assert_eq!(identify_todo_comment(comment), Some("DEBUG".to_string()));
}

#[tokio::test]
async fn test_secondary_keyword_not_found_in_middle() {
    let comment = "// This is a REVIEW comment.";
    assert_eq!(identify_todo_comment(comment), None);
}

#[tokio::test]
async fn test_comment_without_keywords() {
    let comment = "// Just a regular comment.";
    assert_eq!(identify_todo_comment(comment), None);
}

#[tokio::test]
async fn test_primary_keyword_with_punctuation() {
    let comment = "// bla bla -- TODO: fix this.";
    assert_eq!(identify_todo_comment(comment), Some("TODO".to_string()));
}

#[tokio::test]
async fn test_secondary_keyword_case_insensitive() {
    let comment = "// fixme: case insensitive.";
    assert_eq!(identify_todo_comment(comment), Some("FIXME".to_string()));
}

#[tokio::test]
async fn test_primary_keyword_with_dot() {
    let comment = "// This comment has a todo in the middle.";
    assert_eq!(identify_todo_comment(comment), Some("TODO".to_string()));
}

#[tokio::test]
async fn test_secondary_keyword_with_case_variation() {
    let comment = "// FiXmE: Mixed case keyword at the start.";
    assert_eq!(identify_todo_comment(comment), Some("FIXME".to_string()));
}

#[tokio::test]
async fn test_no_keyword_found() {
    let comment = "// This is just a comment without anything.";
    assert_eq!(identify_todo_comment(comment), None);
}

#[tokio::test]
async fn test_no_keyword_found_with_similar_primary_word() {
    let comment = "// I love todoctor";
    assert_eq!(identify_todo_comment(comment), None);
}

#[tokio::test]
async fn test_no_keyword_found_with_similar_secondary_word() {
    let comment = "// Dangerous stuff";
    assert_eq!(identify_todo_comment(comment), None);
}
