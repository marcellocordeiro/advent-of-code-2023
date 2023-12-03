pub fn split_by_line(input: &str) -> Vec<&str> {
    input.trim().split('\n').collect()
}
