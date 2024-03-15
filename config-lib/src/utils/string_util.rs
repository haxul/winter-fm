pub fn has_space_char(s: &str) -> bool {
    for ch in s.chars() {
        if ch.is_whitespace() {
            return true;
        }
    }

    false
}