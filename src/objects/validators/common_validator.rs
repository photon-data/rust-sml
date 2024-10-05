use regex::Regex;
pub fn validate_version(version: &str) -> Result<String, String> {
    let version_regex = Regex::new(r"^(\d+)\.(\d+)\.(\d+)$").unwrap();
    if !version_regex.is_match(version) {
        return Err("Version must be in the format MAJOR.MINOR.PATCH (e.g., 1.0.0).".to_string());
    }
    Ok(version.to_string()) // Return the validated version
}