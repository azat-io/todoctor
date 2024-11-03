use crate::comments::{
    get_comments, identify_not_ignored_file, identify_supported_file,
    identify_todo_comment,
};
use crate::git::{
    get_file_by_commit, get_files_list, get_history, get_last_commit_hash,
    get_modified_files,
};
use crate::types::TodoHistory;
use crate::utils::{add_missing_days, remove_duplicate_dates};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn collect_todo_history(
    months: u32,
    ignores: &[String],
    include_keywords: &[String],
    exclude_keywords: &[String],
    todo_counts: Arc<Mutex<HashMap<String, usize>>>,
) -> Vec<TodoHistory> {
    let mut todo_history_data: Vec<TodoHistory> = Vec::new();

    let mut history: Vec<(String, String)> = get_history(Some(months)).await;
    history = remove_duplicate_dates(history);

    if history.len() > 1 {
        history.remove(0);
    }

    let history_len = history.len();

    let progress_bar = ProgressBar::new(history_len as u64);

    let progress_style = ProgressStyle::default_bar()
        .template("{bar:40.cyan/blue} {pos}/{len} ({percent}%)")
        .expect("Failed to create progress bar template")
        .progress_chars("▇▇ ");
    progress_bar.set_style(progress_style);

    let mut previous_commit_hash: Option<String> = get_last_commit_hash().await;

    for (_index, (commit_hash, date)) in history.iter().enumerate() {
        progress_bar.inc(1);

        io::stdout().flush().unwrap();

        let files_list =
            get_files_list(Some(commit_hash.as_str())).await.unwrap();

        let supported_files: Vec<_> = files_list
            .clone()
            .into_iter()
            .filter(|file| {
                identify_not_ignored_file(file, ignores)
                    && identify_supported_file(file)
            })
            .collect();

        let modified_files = if let Some(prev_hash) = &previous_commit_hash {
            get_modified_files(prev_hash, commit_hash)
                .await
                .into_iter()
                .filter(|file| {
                    identify_not_ignored_file(file, ignores)
                        && identify_supported_file(file)
                })
                .collect::<Vec<_>>()
        } else {
            supported_files.clone()
        };

        for file_path in &modified_files {
            let file_content_result =
                get_file_by_commit(commit_hash, file_path).await;

            match file_content_result {
                Ok(file_content) => {
                    let comments = get_comments(&file_content, file_path);

                    let include_keywords_refs: Vec<&str> =
                        include_keywords.iter().map(|s| s.as_str()).collect();
                    let exclude_keywords_refs: Vec<&str> =
                        exclude_keywords.iter().map(|s| s.as_str()).collect();

                    let todos: Vec<_> = comments
                        .into_iter()
                        .filter(|comment| {
                            identify_todo_comment(
                                &comment.text,
                                Some(&include_keywords_refs),
                                Some(&exclude_keywords_refs),
                            )
                            .is_some()
                        })
                        .collect();

                    let new_count = todos.len();

                    {
                        let mut counts = todo_counts.lock().await;
                        if new_count > 0 {
                            counts.insert(file_path.clone(), new_count);
                        } else {
                            counts.remove(file_path);
                        }
                    }
                }
                Err(_) => {
                    let mut counts = todo_counts.lock().await;
                    counts.remove(file_path);
                }
            };
        }

        let counts = todo_counts.lock().await;
        let total_todo_count: usize = counts.values().sum();
        drop(counts);

        todo_history_data.push(TodoHistory {
            date: date.clone(),
            count: total_todo_count,
        });

        previous_commit_hash = Some(commit_hash.clone());
    }

    progress_bar.finish_with_message("All commits processed!");

    let current_total = {
        let counts = todo_counts.lock().await;
        counts.values().sum()
    };

    todo_history_data =
        add_missing_days(todo_history_data, months, current_total);

    todo_history_data
}
