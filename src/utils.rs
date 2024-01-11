/// Get the value from [`std::env::var`] variable if, it's doesn't exist set it into default value
/// and return
pub fn env(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(_err) => {
            std::env::set_var(key, default);
            default.to_string()
        }
    }
}
