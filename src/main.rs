use futures::future::join_all;
use open;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::Arc;
use todoctor::blame::blame;
use todoctor::check_git_repository::check_git_repository;
use todoctor::copy_dir_recursive::copy_dir_recursive;
use todoctor::exec::exec;
use todoctor::get_comments::get_comments;
use todoctor::get_current_directory::get_current_directory;
use todoctor::get_current_exe_path::get_current_exe_path;
use todoctor::get_history::get_history;
use todoctor::get_line_from_position::get_line_from_position;
use todoctor::get_project_name::get_project_name;
use todoctor::get_todoctor_version::get_todoctor_version;
use todoctor::identify_supported_file::identify_supported_file;
use todoctor::identify_todo_comment::identify_todo_comment;
use todoctor::prepare_blame_data::{prepare_blame_data, PreparedBlameData};
use todoctor::todo_keywords::TODO_KEYWORDS;
use tokio::fs;
use tokio::sync::Semaphore;

#[derive(Debug, Serialize)]
pub struct TodoData {
    pub comment: String,
    pub path: String,
    pub start: usize,
    pub end: usize,
    pub kind: String,
}

#[derive(Debug, Serialize)]
pub struct TodoWithBlame {
    pub blame: PreparedBlameData,
    pub comment: String,
    pub path: String,
    pub line: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoHistory {
    pub todos_count: usize,
    pub commit: String,
    pub date: String,
}

const TODOCTOR_DIR: &str = "todoctor";
const TODO_JSON_FILE: &str = "todoctor/data.json";

#[tokio::main]
async fn main() {
    if !check_git_repository(".").await {
        eprintln!("Error: This is not a Git repository.");
        process::exit(1);
    }

    let file_list = exec(&["git", "ls-files"]).await.unwrap();

    let files: Vec<String> = file_list
        .lines()
        .filter(|file: &&str| identify_supported_file(file))
        .map(|file: &str| file.to_string())
        .collect();

    let todo_data_tasks: Vec<_> = files
        .into_iter()
        .map(|source_file_name: String| {
            tokio::spawn(async move {
                match fs::read_to_string(&source_file_name).await {
                    Ok(source) => {
                        let comments = get_comments(&source, &source_file_name);
                        comments
                            .into_iter()
                            .filter(|comment| identify_todo_comment(&comment.text))
                            .map(move |comment| TodoData {
                                path: source_file_name.clone(),
                                comment: comment.text.clone(),
                                start: comment.start,
                                end: comment.end,
                                kind: format!("{:?}", comment.kind),
                            })
                            .collect::<Vec<TodoData>>()
                    }
                    Err(e) => {
                        eprintln!("Error reading file {}: {:?}", source_file_name, e);
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
                    if let Some(line) = get_line_from_position(todo.start, &source_code) {
                        if let Some(blame_data) = blame(&todo.path, line).await {
                            let prepared_blame: PreparedBlameData = prepare_blame_data(blame_data);
                            return Some(TodoWithBlame {
                                comment: todo.comment.trim().to_string(),
                                path: todo.path.clone(),
                                blame: prepared_blame,
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

    let mut history: Vec<(String, String)> = get_history().await;
    let mut todo_count: usize = todos_with_blame.len();

    let temp_file = File::create("todo_history_temp.json").expect("Failed to create temp file");
    let mut writer = BufWriter::new(temp_file);

    if history.len() > 1 {
        history.remove(0);
    }

    let history_len = history.len();
    let semaphore = Arc::new(Semaphore::new(5));

    for (index, (commit_hash, date)) in history.iter().enumerate() {
        print!("\rProcessed {}/{} commits", index, history_len);
        io::stdout().flush().unwrap();

        if index + 1 < history.len() {
            let prev_commit = &history[index + 1].0;

            if let Ok(diff_stat_output) =
                exec(&["git", "diff", "--stat", prev_commit, commit_hash]).await
            {
                let supported_files: Vec<_> = diff_stat_output
                    .lines()
                    .filter_map(|line| {
                        let file_path = line.split_whitespace().next()?;
                        if identify_supported_file(file_path) {
                            Some(file_path.to_string())
                        } else {
                            None
                        }
                    })
                    .collect();

                let file_tasks: Vec<_> = supported_files
                    .into_iter()
                    .map(|file_path| {
                        let semaphore = semaphore.clone();
                        let commit_hash = commit_hash.clone();
                        tokio::spawn(async move {
                            let permit = semaphore.acquire_owned().await.unwrap();
                            if let Ok(diff_output) =
                                exec(&["git", "diff", commit_hash.as_str(), "--", &file_path]).await
                            {
                                for line in diff_output.lines() {
                                    if line.starts_with("+") || line.starts_with("-") {
                                        let line_uppercase = line.to_uppercase();

                                        let added = line.starts_with("+")
                                            && TODO_KEYWORDS.iter().any(|keyword| {
                                                line_uppercase.contains(&keyword.to_uppercase())
                                            });
                                        let removed = line.starts_with("-")
                                            && TODO_KEYWORDS.iter().any(|keyword| {
                                                line_uppercase.contains(&keyword.to_uppercase())
                                            });

                                        if added {
                                            return Some((true, file_path));
                                        }

                                        if removed {
                                            return Some((false, file_path));
                                        }
                                    }
                                }
                            }
                            drop(permit);
                            None
                        })
                    })
                    .collect();

                let results = join_all(file_tasks).await;

                for result in results {
                    if let Ok(Some((added, _file_path))) = result {
                        if added {
                            todo_count += 1;
                        } else {
                            todo_count -= 1;
                        }
                    }
                }

                let todo_history = TodoHistory {
                    commit: commit_hash.clone(),
                    todos_count: todo_count,
                    date: date.clone(),
                };

                let json_entry = serde_json::to_string(&todo_history).expect("Failed to serialize");
                writeln!(writer, "{}", json_entry).expect("Failed to write to temp file");
            }
        }
    }

    writer.flush().expect("Failed to flush writer");

    println!("\nFound {} todos", todo_count);

    let temp_file = File::open("todo_history_temp.json").expect("Failed to open temp file");
    let reader = BufReader::new(temp_file);

    for line in reader.lines() {
        let entry: TodoHistory = serde_json::from_str(&line.expect("Error reading line"))
            .expect("Error deserializing JSON");
        println!(
            "Date: {}, TODO count: {}, Commit {}",
            entry.date, entry.todos_count, entry.commit
        );
    }

    if fs::metadata(TODOCTOR_DIR).await.is_ok() {
        fs::remove_dir_all(TODOCTOR_DIR)
            .await
            .expect("Error: Failed to remove directory");
    }
    fs::create_dir_all(TODOCTOR_DIR)
        .await
        .expect("Error creating directory");

    let current_directory =
        get_current_directory().expect("Error: Could not get current directory.");
    let project_name = get_project_name().unwrap_or_else(|| "Unknown Project".to_string());
    let version = get_todoctor_version()
        .await
        .unwrap_or_else(|| "Unknown Version".to_string());

    let json_data = json!({
        "currentPath": current_directory,
        "data": todos_with_blame,
        "name": project_name,
        "version": version,
    });

    let json_string: String = serde_json::to_string(&json_data).expect("Error serializing data");

    let current_exe_path: PathBuf =
        get_current_exe_path().expect("Error: Could not get current exe path.");
    let script_dir: &Path = current_exe_path
        .parent()
        .expect("Error: Could not get script directory.");
    let dist_path: PathBuf = script_dir.join("../dist");

    copy_dir_recursive(&dist_path, Path::new(TODOCTOR_DIR))
        .await
        .expect("Error copying directory");

    let index_path = Path::new(TODOCTOR_DIR).join("index.html");
    if fs::metadata(&index_path).await.is_ok() {
        let mut index_content = fs::read_to_string(&index_path)
            .await
            .expect("Error reading index.html");

        if let Some(pos) = index_content.find("</head>") {
            let script_tag: String = format!("<script>window.data = {};</script>", json_string);
            index_content.insert_str(pos, &script_tag);

            fs::write(&index_path, index_content)
                .await
                .expect("Error writing modified index.html");
        } else {
            eprintln!("Error: No </head> tag found in index.html");
        }
    }

    if let Err(e) = open::that(&index_path) {
        eprintln!("Error: Cannot open index.html: {:?}", e);
    }

    println!("Data successfully written to {}", TODO_JSON_FILE);
}
