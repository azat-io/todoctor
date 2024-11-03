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
