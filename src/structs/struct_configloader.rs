// D:/Work/Projects/Modding/Skyrim/dds-edit/src/structs/config_loader.rs

use super::struct_config::Config;
use std::fs;
use toml;

/// Loads configuration from a TOML file.
///
/// # Arguments
///
/// * `path` - The path to the TOML configuration file.
///
/// # Returns
///
/// * `Ok(Config)` if loading and parsing succeed.
/// * `Err(String)` with an error message if loading or parsing fails.
pub fn load_config_from_file(path: &str) -> Result<Config, String> {
    let config_content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config file '{}': {}", path, e))?;
    toml::from_str(&config_content)
        .map_err(|e| format!("Failed to parse config file '{}': {}", path, e))
}
