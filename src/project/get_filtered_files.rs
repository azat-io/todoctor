use crate::comments::{identify_not_ignored_file, identify_supported_file};
use crate::git::get_files_list;

pub async fn get_filtered_files(ignores: &[String]) -> Vec<String> {
    let files_list = get_files_list(None).await.unwrap();

    files_list
        .into_iter()
        .filter(|file| {
            identify_not_ignored_file(file, ignores)
                && identify_supported_file(file)
        })
        .collect()
}
