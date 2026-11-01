//! Tiny fixture for Robby's bootstrap.

/// Loads a config file, failing on empty input.
pub fn load(path: &str) -> Result<String, Error> {
    let raw = std::fs::read_to_string(path)?;
    if raw.is_empty() {
        return Err(Error::Empty);
    }
    Ok(raw)
}

/// Loads a config and reports its length.
pub fn load_len(path: &str) -> Result<usize, Error> {
    let raw = load(path)?;
    Ok(raw.len())
}
