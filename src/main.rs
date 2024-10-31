use clap::{ArgAction, CommandFactory, Parser, ValueEnum};
use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use open;
use serde_json::{json, Value};
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::Arc;
use todoctor::add_missing_days::add_missing_days;
use todoctor::blame::blame;
use todoctor::check_git_repository::check_git_repository;
use todoctor::copy_dir_recursive::copy_dir_recursive;
use todoctor::escape_json_values::escape_json_values;
use todoctor::exec::exec;
use todoctor::get_comments::get_comments;
use todoctor::get_current_directory::get_current_directory;
use todoctor::get_dist_path::get_dist_path;
use todoctor::get_files_list::get_files_list;
use todoctor::get_history::get_history;
use todoctor::get_line_from_position::get_line_from_position;
use todoctor::get_project_name::get_project_name;
use todoctor::get_todoctor_version::get_todoctor_version;
use todoctor::identify_not_ignored_file::identify_not_ignored_file;
use todoctor::identify_supported_file::identify_supported_file;
use todoctor::identify_todo_comment::identify_todo_comment;
use todoctor::prepare_blame_data::{prepare_blame_data, PreparedBlameData};
use todoctor::remove_duplicate_dates::remove_duplicate_dates;
use todoctor::types::{TodoData, TodoHistory, TodoWithBlame};
use tokio::fs;
use tokio::sync::Semaphore;

const HISTORY_TEMP_FILE: &str = "todo_history_temp.json";

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
    #[arg(short, long, default_value = "html")]
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

    let todo_data_tasks: Vec<_> = files
        .into_iter()
        .map(|source_file_name: String| {
            let include_keywords = include_keywords.clone();
            let exclude_keywords = exclude_keywords.clone();

            tokio::spawn(async move {
                match fs::read_to_string(&source_file_name).await {
                    Ok(source) => {
                        let comments = get_comments(&source, &source_file_name);
                        comments
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
                            .collect::<Vec<TodoData>>()
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

    let temp_file =
        File::create(HISTORY_TEMP_FILE).expect("Failed to create temp file");
    let mut writer = BufWriter::new(temp_file);

    if history.len() > 1 {
        history.remove(0);
    }

    let history_len = history.len();
    let semaphore = Arc::new(Semaphore::new(24));

    let progress_bar = ProgressBar::new(history_len as u64);

    let progress_style = ProgressStyle::default_bar()
        .template("{bar:40.cyan/blue} {pos}/{len} ({percent}%)")
        .expect("Failed to create progress bar template")
        .progress_chars("▇▇ ");
    progress_bar.set_style(progress_style);

    for (_index, (commit_hash, date)) in history.iter().enumerate() {
        progress_bar.inc(1);

        io::stdout().flush().unwrap();

        let files_list =
            get_files_list(Some(commit_hash.as_str())).await.unwrap();

        let supported_files: Vec<_> = files_list
            .into_iter()
            .filter(|file| {
                identify_not_ignored_file(file, &ignores)
                    && identify_supported_file(file)
            })
            .collect();

        let file_tasks: Vec<_> = supported_files
            .into_iter()
            .map(|file_path| {
                let semaphore = semaphore.clone();
                let commit_hash = commit_hash.clone();

                let include_keywords = include_keywords.clone();
                let exclude_keywords = exclude_keywords.clone();

                tokio::spawn(async move {
                    let permit = semaphore.acquire_owned().await.unwrap();

                    if let Ok(file_content) = exec(&[
                        "git",
                        "show",
                        &format!("{}:{}", commit_hash, file_path),
                    ])
                    .await
                    {
                        let comments = get_comments(&file_content, &file_path);

                        let include_keywords_refs: Vec<&str> = include_keywords
                            .iter()
                            .map(|s| s.as_str())
                            .collect();
                        let exclude_keywords_refs: Vec<&str> = exclude_keywords
                            .iter()
                            .map(|s| s.as_str())
                            .collect();

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

                        drop(permit);
                        Some(todos.len())
                    } else {
                        drop(permit);
                        None
                    }
                })
            })
            .collect();

        let results = join_all(file_tasks).await;

        let todo_count: usize = results
            .into_iter()
            .filter_map(|res| res.ok().flatten())
            .sum();

        let todo_history = TodoHistory {
            date: date.clone(),
            count: todo_count,
        };

        let json_entry =
            serde_json::to_string(&todo_history).expect("Failed to serialize");
        writeln!(writer, "{}", json_entry)
            .expect("Failed to write to temp file");
    }

    progress_bar.finish_with_message("All commits processed!");

    writer.flush().expect("Failed to flush writer");

    if fs::metadata(output_directory).await.is_ok() {
        fs::remove_dir_all(output_directory)
            .await
            .expect("Error: Failed to remove directory");
    }
    fs::create_dir_all(output_directory)
        .await
        .expect("Error creating directory");

    let current_directory = get_current_directory()
        .expect("Error: Could not get current directory.");
    let project_name =
        get_project_name().unwrap_or_else(|| "Unknown Project".to_string());

    let file = File::open(HISTORY_TEMP_FILE).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut todo_history_data: Vec<TodoHistory> = Vec::new();
    for line in reader.lines() {
        let entry: TodoHistory =
            serde_json::from_str(&line.expect("Error reading line"))
                .expect("Error deserializing JSON");
        todo_history_data.push(entry);
    }
    todo_history_data =
        add_missing_days(todo_history_data, *months, todos_with_blame.len());

    fs::remove_file(HISTORY_TEMP_FILE)
        .await
        .expect("Error: Failed to remove temporary file");

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
