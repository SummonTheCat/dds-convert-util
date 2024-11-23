// D:/Work/Projects/Modding/Skyrim/dds-edit/src/main.rs

use crate::{
    io::io_out::setup_output_directory,
    io::io_pathtree::{filter_pathtree_types, get_path_tree},
    logging::{smnprint, smnprintln},
    processing::processing_core::process_files,
};
use cli::cli_args::cli_args::parse_arguments;
use colored::Color;
use io::io_pathtree::print_path_tree;
use structs::{struct_config::Config, struct_configloader::load_config_from_file};

mod structs;
mod cli;
mod io;
mod processing;
mod logging;

#[tokio::main]
async fn main() {
    // Parse and validate command-line arguments.
    let cli_config = match parse_arguments() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Initialize the configuration with defaults.
    let mut config = Config::default();

    // If --useconfig is set, load configurations from the specified file.
    if let Some(config_file_path) = &cli_config.config_file {
        match load_config_from_file(config_file_path) {
            Ok(file_config) => {
                // Merge configurations: CLI args override file config.
                config = merge_configs(file_config, cli_config);
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    } else {
        // If no config file, use CLI arguments.
        config = merge_configs(config, cli_config);
    }

    // Validate that the input path is provided.
    if config.path_in.is_empty() {
        eprintln!("Error: Input path is required.");
        std::process::exit(1);
    }

    // Log the parsed configuration parameters.
    log_parameters(&config);

    // Setup the output directory.
    if let Err(e) = setup_output_directory(&config.path_out) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    // Get all file paths up to the specified max_depth.
    let all_paths = match get_path_tree(&config.path_in, config.max_depth) {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Define supported file types.
    let supported_types = vec!["jpg", "jpeg", "png"];

    // Filter paths to include only supported image types.
    let filtered_paths = filter_pathtree_types(&all_paths, &supported_types);

    if filtered_paths.is_empty() {
        eprintln!("No supported image files found in '{}'.", config.path_in);
        std::process::exit(0);
    }

    print_path_tree(&filtered_paths, config.max_depth);

    // Process the filtered image files.
    if let Err(e) = process_files(&filtered_paths, &config).await {
        eprintln!("Error during processing: {}", e);
        std::process::exit(1);
    }
}

/// Merges configurations from the file and CLI arguments.
/// CLI arguments take precedence over file configurations.
///
/// # Arguments
///
/// * `file_config` - Configuration loaded from the file.
/// * `cli_config` - Configuration loaded from CLI arguments.
///
/// # Returns
///
/// * `Config` - Merged configuration.
fn merge_configs(file_config: Config, cli_config: Config) -> Config {
    Config {
        path_in: if !cli_config.path_in.is_empty() {
            cli_config.path_in
        } else {
            file_config.path_in
        },
        path_out: if !cli_config.path_out.is_empty() && cli_config.path_out != "./images/output" {
            cli_config.path_out
        } else {
            file_config.path_out
        },
        max_depth: if cli_config.max_depth != usize::MAX {
            cli_config.max_depth
        } else {
            file_config.max_depth
        },
        target_sizes: if cli_config.target_sizes != vec![2048] {
            cli_config.target_sizes
        } else {
            file_config.target_sizes
        },
        threads: if cli_config.threads != 4 {
            cli_config.threads
        } else {
            file_config.threads
        },
        config_file: cli_config.config_file, // Retain the config_file field from CLI if needed
    }
}

/// Logs the configuration parameters to the console with color coding.
///
/// # Arguments
///
/// * `config` - Reference to the `Config` struct containing configuration parameters.
fn log_parameters(config: &Config) {
    smnprintln("Configuration Parameters:", Color::Yellow);
    smnprint("Input Path: ", Color::White);
    smnprintln(&config.path_in, Color::BrightCyan);
    smnprint("Output Path: ", Color::White);
    smnprintln(&config.path_out, Color::BrightCyan);
    smnprint("Max Depth: ", Color::White);
    smnprintln(&config.max_depth.to_string(), Color::BrightCyan);
    smnprint("Threads: ", Color::White);
    smnprintln(&config.threads.to_string(), Color::BrightCyan);

    smnprint("Width Targets: ", Color::White);
    for (i, size) in config.target_sizes.iter().enumerate() {
        if i > 0 {
            smnprint(", ", Color::White);
        }
        smnprint(size.to_string(), Color::BrightCyan);
    }
    println!();
}
