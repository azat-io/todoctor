use regex::Regex;

pub fn identify_supported_file(file_name: &str) -> bool {
    let re = Regex::new(r"\.[cm]?[jt]sx?$").unwrap();
    re.is_match(file_name)
}
