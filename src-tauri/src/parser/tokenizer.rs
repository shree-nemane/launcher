pub fn tokenize(input: &str) -> Vec<String> {
    input
        .trim()
        .split_whitespace()
        .map(|s| s.to_lowercase())
        .collect()
}
