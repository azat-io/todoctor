use crate::escape_html::escape_html;
use serde_json::Value;

pub fn escape_json_values(json_value: &mut Value) {
    match json_value {
        Value::String(s) => {
            let escaped = escape_html(s);
            *s = escaped;
        }
        Value::Array(arr) => {
            for item in arr {
                escape_json_values(item);
            }
        }
        Value::Object(map) => {
            for (_key, value) in map {
                escape_json_values(value);
            }
        }
        _ => {}
    }
}
