pub fn strip_str(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}
