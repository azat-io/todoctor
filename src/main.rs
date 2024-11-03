use clap::{ArgAction, CommandFactory, Parser, ValueEnum};
use indicatif::{ProgressBar, ProgressStyle};
use open;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::Arc;
use todoctor::add_missing_days::add_missing_days;
use todoctor::blame::blame;
use todoctor::check_git_repository::check_git_repository;
use todoctor::copy_dir_recursive::copy_dir_recursive;
use todoctor::escape_json_values::escape_json_values;
use todoctor::get_comments::get_comments;
use todoctor::get_current_directory::get_current_directory;
use todoctor::get_dist_path::get_dist_path;
use todoctor::get_file_by_commit::get_file_by_commit;
use todoctor::get_files_list::get_files_list;
use todoctor::get_history::get_history;
use todoctor::get_last_commit_hash::get_last_commit_hash;
use todoctor::get_line_from_position::get_line_from_position;
use todoctor::get_modified_files::get_modified_files;
use todoctor::get_project_name::get_project_name;
use todoctor::get_todoctor_version::get_todoctor_version;
use todoctor::identify_not_ignored_file::identify_not_ignored_file;
use todoctor::identify_supported_file::identify_supported_file;
use todoctor::identify_todo_comment::identify_todo_comment;
use todoctor::prepare_blame_data::{prepare_blame_data, PreparedBlameData};
use todoctor::remove_duplicate_dates::remove_duplicate_dates;
use todoctor::types::{TodoData, TodoHistory, TodoWithBlame};
use tokio::fs;
use tokio::sync::Mutex;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutputFormat {
    Html,
    Json,
}

#[derive(Parser, Debug)]
#[command(
    name = "todoctor",
    about = "
Todoctor is a powerful tool for analyzing, tracking, and visualizing technical
debt in your codebase using Git. It collects and monitors TODO/FIXME comments
in your code, allowing you to observe changes over time."
)]
struct Cli {
    /// Number of months to process
    #[arg(short, long, default_value_t = 3, value_parser = clap::value_parser!(u32).range(1..))]
    month: u32,

    /// Paths to ignore (can be used multiple times)
    #[arg(short, long, action = ArgAction::Append)]
    ignore: Vec<String>,

    /// Keywords to track for TODO comments (can be used multiple times)
    #[arg(short = 'I', long, action = ArgAction::Append)]
    include_keywords: Vec<String>,

    /// Keywords to exclude from tracking (can be used multiple times)
    #[arg(short = 'E', long, action = ArgAction::Append)]
    exclude_keywords: Vec<String>,

    /// Output format
    #[arg(short = 'f', long, default_value = "html")]
    output_format: OutputFormat,

    /// Output directory
    #[arg(short, long, default_value = "todoctor")]
    output: String,
}

#[tokio::main]
async fn main() {
    let version = get_todoctor_version()
        .await
        .unwrap_or_else(|| "Unknown version".to_string());

    let version_for_cli = version.clone();
    let version_static: &'static str =
        Box::leak(version_for_cli.into_boxed_str());

    let args = Cli::command().version(version_static).get_matches();

    let months = args.get_one::<u32>("month").unwrap();
    let ignores: Vec<String> = args
        .get_many::<String>("ignore")
        .map(|values| values.map(String::from).collect())
        .unwrap_or_else(Vec::new);

    let include_keywords: Vec<String> = args
        .get_many::<String>("include_keywords")
        .map(|values| values.map(String::from).collect())
        .unwrap_or_else(Vec::new);

    let exclude_keywords: Vec<String> = args
        .get_many::<String>("exclude_keywords")
        .map(|values| values.map(String::from).collect())
        .unwrap_or_else(Vec::new);

    let output_format = args.get_one::<OutputFormat>("output_format").unwrap();

    let output_directory = args.get_one::<String>("output").unwrap();

    if !check_git_repository(".").await {
        eprintln!("Error: This is not a Git repository.");
        process::exit(1);
    }

    let files_list = get_files_list(None).await.unwrap();

    let files: Vec<String> = files_list
        .into_iter()
        .filter(|file| {
            identify_not_ignored_file(file, &ignores)
                && identify_supported_file(file)
        })
        .collect();

    let todo_counts = Arc::new(Mutex::new(HashMap::new()));
    let todo_counts_clone = Arc::clone(&todo_counts);

    let mut todo_history_data: Vec<TodoHistory> = Vec::new();

    let todo_data_tasks: Vec<_> = files
        .into_iter()
        .map(|source_file_name: String| {
            let include_keywords = include_keywords.clone();
            let exclude_keywords = exclude_keywords.clone();
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

                        if todos.len() > 0 {
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

    let counts = todo_counts.lock().await;
    drop(counts);

    let todo_with_blame_tasks: Vec<_> = todo_data
        .into_iter()
        .map(|todo| {
            tokio::spawn(async move {
                if let Ok(source_code) = fs::read_to_string(&todo.path).await {
                    if let Some(line) =
                        get_line_from_position(todo.start, &source_code)
                    {
                        if let Some(blame_data) = blame(&todo.path, line).await
                        {
                            let prepared_blame: PreparedBlameData =
                                prepare_blame_data(blame_data);
                            return Some(TodoWithBlame {
                                comment: todo.comment.trim().to_string(),
                                path: todo.path.clone(),
                                blame: prepared_blame,
                                kind: todo.kind,
                                line,
                            });
                        }
                    }
                }
                None
            })
        })
        .collect();

    let mut todos_with_blame: Vec<TodoWithBlame> = Vec::new();
    for task in todo_with_blame_tasks {
        if let Ok(Some(todo)) = task.await {
            todos_with_blame.push(todo);
        }
    }

    todos_with_blame.sort_by(|a, b| a.blame.date.cmp(&b.blame.date));

    let mut history: Vec<(String, String)> = get_history(Some(*months)).await;
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
                identify_not_ignored_file(file, &ignores)
                    && identify_supported_file(file)
            })
            .collect();

        let modified_files = if let Some(prev_hash) = &previous_commit_hash {
            get_modified_files(prev_hash, &commit_hash)
                .await
                .into_iter()
                .filter(|file| {
                    identify_not_ignored_file(file, &ignores)
                        && identify_supported_file(file)
                })
                .collect::<Vec<_>>()
        } else {
            supported_files.clone()
        };

        for file_path in &modified_files {
            let file_content_result =
                get_file_by_commit(&commit_hash, &file_path).await;

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

    todo_history_data =
        add_missing_days(todo_history_data, *months, todos_with_blame.len());

    let current_directory = get_current_directory()
        .expect("Error: Could not get current directory.");
    let project_name =
        get_project_name().unwrap_or_else(|| "Unknown Project".to_string());

    let json_data = json!({
        "currentPath": current_directory,
        "history": todo_history_data,
        "data": todos_with_blame,
        "name": project_name,
        "version": version,
    });

    generate_output(*output_format, output_directory, json_data).await;
}

async fn generate_output(
    output_format: OutputFormat,
    output_directory: &str,
    json_data: Value,
) {
    match output_format {
        OutputFormat::Html => {
            let dist_path: PathBuf = get_dist_path()
                .expect("Error: Could not get current dist path.");

            copy_dir_recursive(&dist_path, Path::new(output_directory))
                .await
                .expect("Error copying directory");

            let mut escaped_json_data = json_data.clone();
            escape_json_values(&mut escaped_json_data);

            let escaped_json_string = serde_json::to_string(&escaped_json_data)
                .expect("Error: Could not serializing JSON");

            let index_path = Path::new(output_directory).join("index.html");
            if fs::metadata(&index_path).await.is_ok() {
                let mut index_content = fs::read_to_string(&index_path)
                    .await
                    .expect("Error reading index.html");

                if let Some(pos) = index_content.find("</head>") {
                    let script_tag = format!(
                        "<script>window.data = {};</script>",
                        escaped_json_string
                    );
                    index_content.insert_str(pos, &script_tag);

                    fs::write(&index_path, index_content)
                        .await
                        .expect("Error writing modified index.html");
                } else {
                    eprintln!("Error: No </head> tag found in index.html");
                }

                if let Err(e) = open::that(&index_path) {
                    eprintln!("Error: Cannot open index.html: {:?}", e);
                }
            }
        }
        OutputFormat::Json => {
            let json_path = Path::new(output_directory).join("report.json");
            let mut file = File::create(&json_path)
                .expect("Failed to create JSON report file");
            let formatted_json = serde_json::to_string_pretty(&json_data)
                .expect("Failed to format JSON data");

            file.write_all(formatted_json.as_bytes())
                .expect("Failed to write JSON data");
        }
    }
}
