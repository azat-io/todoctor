use clap::{ArgAction, CommandFactory, Parser};
use todoctor::project::{
    check_git_repository_or_exit, collect_todo_data, collect_todo_history,
    enrich_todo_data_with_blame, generate_output, get_filtered_files,
    get_todoctor_version, prepare_json_data,
};
use todoctor::types::OutputFormat;

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

    let args = parse_args(&version);

    check_git_repository_or_exit().await;

    let files = get_filtered_files(&args.ignore).await;

    let (todo_data, todo_counts) = collect_todo_data(
        &files,
        &args.include_keywords,
        &args.exclude_keywords,
    )
    .await;

    let todos_with_blame = enrich_todo_data_with_blame(todo_data).await;

    let todo_history_data = collect_todo_history(
        args.month,
        &args.ignore,
        &args.include_keywords,
        &args.exclude_keywords,
        todo_counts,
    )
    .await;

    let json_data =
        prepare_json_data(&todo_history_data, &todos_with_blame, &version)
            .await;

    generate_output(args.output_format, &args.output, json_data).await;
}

fn parse_args(version: &str) -> Cli {
    use clap::FromArgMatches;

    let version_static: &'static str =
        Box::leak(version.to_string().into_boxed_str());

    let mut cmd = Cli::command();
    cmd = cmd.version(version_static);
    let matches = cmd.get_matches();

    match Cli::from_arg_matches(&matches) {
        Ok(cli) => cli,
        Err(e) => e.exit(),
    }
}
