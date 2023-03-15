pub fn match_lowercase(target: &str, names: &[&str]) -> bool {
    for name in names {
        if target.eq_ignore_ascii_case(name) {
            return true;
        }
    }
    false
}
