pub fn to_str_refs(values: &[String]) -> Vec<&str> {
    values.iter().map(|value| value.as_str()).collect()
}
