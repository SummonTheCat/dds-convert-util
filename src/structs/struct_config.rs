// D:/Work/Projects/Modding/Skyrim/dds-edit/src/structs/struct_config.rs

use serde::Deserialize;

/// Structure to hold parsed command-line arguments or configuration file parameters.
///
/// Contains all configuration parameters required for processing images.
#[derive(Clone, Deserialize)]
pub struct Config {
    /// Input directory path containing images to be processed.
    pub path_in: String,

    /// Output directory path where processed images will be saved.
    pub path_out: String,

    /// Maximum directory traversal depth.
    pub max_depth: usize,

    /// List of target widths for resizing images.
    pub target_sizes: Vec<u32>,

    /// Number of threads to use for concurrent processing.
    pub threads: usize,

    /// Optional path to the configuration file.
    #[serde(skip)]
    pub config_file: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path_in: String::new(),
            path_out: "./images/output".to_string(),
            max_depth: usize::MAX,
            target_sizes: vec![2048],
            threads: 4,
            config_file: None,
        }
    }
}
