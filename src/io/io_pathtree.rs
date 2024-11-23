// D:/Work/Projects/Modding/Skyrim/dds-edit/src/io/io_pathtree.rs

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

/// Recursively retrieves all paths within `base_path` up to `max_depth`.
///
/// # Arguments
///
/// * `base_path` - The root directory to start traversal.
/// * `max_depth` - The maximum depth for recursive traversal.
///
/// # Returns
///
/// * `Ok(Vec<PathBuf>)` containing all collected paths.
/// * `Err(Box<dyn std::error::Error>)` if an error occurs during traversal.
pub fn get_path_tree(base_path: &str, max_depth: usize) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut paths = Vec::new();
    let base = Path::new(base_path);

    // Check if the base path exists.
    if !base.exists() {
        return Err(format!("Path '{}' does not exist.", base.display()).into());
    }

    // Begin collecting paths recursively.
    collect_paths(base, &mut paths, 0, max_depth)?;
    Ok(paths)
}

/// Recursively collects paths up to the specified depth.
///
/// # Arguments
///
/// * `path` - Current path being traversed.
/// * `paths` - Accumulator for collected paths.
/// * `current_depth` - Current traversal depth.
/// * `max_depth` - Maximum allowed traversal depth.
///
/// # Returns
///
/// * `Ok(())` if traversal succeeds.
/// * `Err(Box<dyn std::error::Error>)` if an error occurs.
fn collect_paths(path: &Path, paths: &mut Vec<PathBuf>, current_depth: usize, max_depth: usize) -> Result<(), Box<dyn std::error::Error>> {
    // Stop recursion if the current depth exceeds max_depth.
    if current_depth > max_depth {
        return Ok(());
    }

    // Add the current path to the list.
    paths.push(path.to_path_buf());

    // If the current path is a directory, traverse its entries.
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            collect_paths(&entry.path(), paths, current_depth + 1, max_depth)?;
        }
    }

    Ok(())
}

/// Filters the collected paths to include only specified file types.
///
/// # Arguments
///
/// * `paths` - Slice of paths to filter.
/// * `extensions` - List of file extensions to include (e.g., `["jpg", "png"]`).
///
/// # Returns
///
/// * `Vec<PathBuf>` containing only paths with the specified extensions.
pub fn filter_pathtree_types(paths: &[PathBuf], extensions: &[&str]) -> Vec<PathBuf> {
    // Create a HashSet of lowercase extensions for efficient lookup.
    let ext_set: HashSet<String> = extensions.iter().map(|&e| e.to_lowercase()).collect();

    // Filter and collect paths that are files with the desired extensions.
    paths
        .iter()
        .filter(|path| {
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    ext_set.contains(&ext.to_lowercase())
                } else {
                    false
                }
            } else {
                false
            }
        })
        .cloned()
        .collect()
}

/// Mirrors the input path structure into the output path.
///
/// # Arguments
///
/// * `base_in` - The root input directory.
/// * `base_out` - The root output directory.
/// * `path_in` - The current path to mirror.
///
/// # Returns
///
/// * `Ok(PathBuf)` representing the mirrored output path.
/// * `Err(Box<dyn std::error::Error>)` if path manipulation fails.
pub fn mirror_path(base_in: &Path, base_out: &Path, path_in: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Calculate the relative path from base_in.
    let relative = path_in.strip_prefix(base_in)?;
    // Join the relative path with base_out to create the mirrored path.
    Ok(base_out.join(relative))
}

/// Prints the directory tree of the filtered paths up to `max_depth`.
///
/// # Arguments
///
/// * `paths` - Slice of paths to print.
/// * `max_depth` - Maximum depth to display.
///
/// # Returns
///
/// * `()`
use colored::Color;
use crate::logging::smnprint;

/// Prints the directory tree of the filtered paths up to `max_depth` with colorized output.
///
/// # Arguments
///
/// * `paths` - Slice of paths to print.
/// * `max_depth` - Maximum depth to display.
pub fn print_path_tree(paths: &[PathBuf], max_depth: usize) {
    for path in paths {
        // Calculate the depth based on the number of components.
        let depth = match path.strip_prefix(".") {
            Ok(rel) => rel.components().count(),
            Err(_) => path.components().count(),
        };

        // Skip paths exceeding the maximum depth.
        if max_depth != 0 && depth > max_depth {
            continue;
        }

        // Generate indentation based on depth.
        let indent = "  ".repeat(depth);
        let file_type = if path.is_dir() { "DIR" } else { "FILE" };

        // Print with colorized output.
        smnprint(format!("{}- [", indent), Color::White); // Indentation and bracket
        smnprint(file_type, Color::BrightBlack); // File type in gray
        smnprint("] ", Color::White); // Closing bracket and space
        smnprint(path.display().to_string(), Color::Cyan); // Path in cyan
        println!(); // End with a newline
    }
}
