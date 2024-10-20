use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SUPPORTED_FILE_REGEX: Regex =
        Regex::new(r"\.[cm]?[jt]sx?$").unwrap();
}

pub fn identify_supported_file(file_name: &str) -> bool {
    SUPPORTED_FILE_REGEX.is_match(file_name)
}
