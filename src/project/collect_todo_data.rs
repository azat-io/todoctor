use crate::comments::{get_comments, identify_todo_comment};
use crate::types::TodoData;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Mutex;

pub async fn collect_todo_data(
    files: &[String],
    include_keywords: &[String],
    exclude_keywords: &[String],
) -> (Vec<TodoData>, Arc<Mutex<HashMap<String, usize>>>) {
    let todo_counts = Arc::new(Mutex::new(HashMap::new()));
    let todo_counts_clone = Arc::clone(&todo_counts);

    let todo_data_tasks: Vec<_> = files
        .iter()
        .cloned()
        .map(|source_file_name| {
            let include_keywords = include_keywords.to_vec();
            let exclude_keywords = exclude_keywords.to_vec();
            let todo_counts = Arc::clone(&todo_counts_clone);

            tokio::spawn(async move {
                match fs::read_to_string(&source_file_name).await {
                    Ok(source) => {
                        let comments = get_comments(&source, &source_file_name);
                        let todos: Vec<TodoData> = comments
                            .into_iter()
                            .filter_map(|comment| {
                                let include_keywords_refs: Vec<&str> =
                                    include_keywords
                                        .iter()
                                        .map(|s| s.as_str())
                                        .collect();
                                let exclude_keywords_refs: Vec<&str> =
                                    exclude_keywords
                                        .iter()
                                        .map(|s| s.as_str())
                                        .collect();

                                if let Some(comment_kind) =
                                    identify_todo_comment(
                                        &comment.text,
                                        Some(&include_keywords_refs),
                                        Some(&exclude_keywords_refs),
                                    )
                                {
                                    Some(TodoData {
                                        path: source_file_name.clone(),
                                        comment: comment.text.clone(),
                                        start: comment.start,
                                        kind: comment_kind,
                                        end: comment.end,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();

                        if !todos.is_empty() {
                            let mut counts = todo_counts.lock().await;
                            counts
                                .insert(source_file_name.clone(), todos.len());
                        }

                        todos
                    }
                    Err(e) => {
                        eprintln!(
                            "Error: Cannot read file {}: {:?}",
                            source_file_name, e
                        );
                        vec![]
                    }
                }
            })
        })
        .collect();

    let mut todo_data: Vec<TodoData> = Vec::new();
    for task in todo_data_tasks {
        if let Ok(result) = task.await {
            todo_data.extend(result);
        }
    }

    (todo_data, todo_counts_clone)
}
