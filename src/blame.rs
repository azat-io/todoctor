use crate::exec::exec;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct BlameData {
    pub author_mail: String,
    pub author_time: String,
    pub author_tz: String,
    pub summary: String,
    pub commit: String,
    pub author: String,
}

pub async fn blame(path: &str, line: u32) -> Option<BlameData> {
    let result = match exec(&[
        "git",
        "blame",
        "--line-porcelain",
        "-L",
        &format!("{},{}", line, line),
        path,
    ])
    .await
    {
        Ok(output) => output,
        Err(error) => {
            println!("Failed to execute git blame: {:?}", error);
            return None;
        }
    };

    let lines: Vec<&str> = result.split('\n').collect();

    let mut blame_data = BlameData {
        author_mail: String::new(),
        author_time: String::new(),
        author_tz: String::new(),
        summary: String::new(),
        commit: String::new(),
        author: String::new(),
    };

    let mut key_map: HashMap<&str, Box<dyn FnMut(&mut BlameData, String)>> =
        HashMap::new();
    key_map.insert("author-mail ", Box::new(|b, v| b.author_mail = v));
    key_map.insert("author-time ", Box::new(|b, v| b.author_time = v));
    key_map.insert("author-tz ", Box::new(|b, v| b.author_tz = v));
    key_map.insert("summary ", Box::new(|b, v| b.summary = v));
    key_map.insert("author ", Box::new(|b, v| b.author = v));

    for current_line in lines {
        for (key, setter) in &mut key_map {
            if current_line.starts_with(key) {
                let value =
                    current_line.replacen(key, "", 1).trim().to_string();
                setter(&mut blame_data, value);
            }
        }

        if current_line.len() >= 40
            && current_line.chars().take(40).all(|c| c.is_ascii_hexdigit())
        {
            blame_data.commit =
                current_line.split_whitespace().next().unwrap().to_string();
        }
    }

    Some(blame_data)
}
