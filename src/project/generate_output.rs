use crate::fs::{copy_dir_recursive, get_dist_path};
use crate::types::OutputFormat;
use crate::utils::escape_json_values;
use open;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tokio::fs;

pub async fn generate_output(
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
