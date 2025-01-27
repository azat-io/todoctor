use crate::fs::{copy_dir_recursive, get_dist_path, make_dir};
use crate::types::OutputFormat;
use crate::utils::escape_json_values;
use csv::Writer;
use open;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tokio::{fs, task};

pub async fn generate_output(
    output_format: OutputFormat,
    output_directory: &str,
    json_data: Value,
) {
    match output_format {
        OutputFormat::Html => {
            let dist_path: PathBuf = get_dist_path()
                .expect("Error: Could not get current dist path.");

            make_dir(output_directory).await;

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
                        "<script id=\"data\" type=\"application/json\">{}</script>",
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
            make_dir(output_directory).await;

            let json_path = Path::new(output_directory).join("index.json");
            let mut file = File::create(&json_path)
                .expect("Failed to create JSON report file");
            let formatted_json = serde_json::to_string_pretty(&json_data)
                .expect("Failed to format JSON data");

            file.write_all(formatted_json.as_bytes())
                .expect("Failed to write JSON data");
        }
        OutputFormat::Csv => {
            make_dir(output_directory).await;

            let csv_path = Path::new(output_directory).join("index.csv");

            let json_data_clone = json_data.clone();

            task::spawn_blocking(move || {
                let file = File::create(&csv_path)
                    .expect("Failed to create CSV report file");

                let mut wtr = Writer::from_writer(file);

                if let Some(data_array) =
                    json_data_clone.get("data").and_then(|d| d.as_array())
                {
                    wtr.write_record(&[
                        "Path",
                        "Line",
                        "Kind",
                        "Comment",
                        "Author",
                        "Date",
                        "Commit Hash",
                    ])
                    .expect("Failed to write CSV headers");

                    for item in data_array {
                        let path = item
                            .get("path")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        let line = item
                            .get("line")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0)
                            .to_string();
                        let kind = item
                            .get("kind")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        let comment = item
                            .get("comment")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");

                        let blame = item.get("blame").unwrap_or(&Value::Null);
                        let author = blame
                            .get("author")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        let date = blame
                            .get("date")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        let hash = blame
                            .get("hash")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");

                        wtr.write_record(&[
                            path, &line, kind, comment, author, date, hash,
                        ])
                        .expect("Failed to write CSV record");
                    }
                } else {
                    eprintln!("No data found in json_data");
                }
                wtr.flush().expect("Failed to flush CSV writer");
            })
            .await
            .expect("Failed to write CSV data");
        }
    }
}
