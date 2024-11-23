// D:/Work/Projects/Modding/Skyrim/dds-edit/src/io/io_out.rs

use crate::structs::struct_filetype::FileType;
use image::DynamicImage;
use std::path::{Path, PathBuf};

/// Sets up the output directory by creating it if it doesn't exist.
///
/// # Arguments
///
/// * `output_path` - The path to the output directory.
///
/// # Returns
///
/// * `Ok(())` if the directory is set up successfully.
/// * `Err(String)` with an error message if it fails.
pub fn setup_output_directory(output_path: &str) -> Result<(), String> {
    let path = Path::new(output_path);
    if !path.exists() {
        std::fs::create_dir_all(path)
            .map_err(|e| format!("Failed to create output directory '{}': {}", output_path, e))?;
    }
    Ok(())
}

/// Saves a resized image with an appropriate suffix and format.
///
/// # Arguments
///
/// * `img` - Reference to the `DynamicImage` to save.
/// * `output_path` - The base output path.
/// * `suffix` - Suffix to append to the file name.
/// * `file_type` - The `FileType` enum indicating the image format.
///
/// # Returns
///
/// * `Ok(PathBuf)` containing the path to the saved image.
/// * `Err(String)` with an error message if saving fails.
pub fn save_image(
    img: &DynamicImage,
    output_path: &Path,
    suffix: &str,
    file_type: FileType, // Changed to accept FileType by value
) -> Result<PathBuf, String> {
    let mut output_path = output_path.to_path_buf();
    let file_stem = output_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let extension = match file_type {
        FileType::JPG => "jpg",
        FileType::PNG => "png",
        FileType::UNKNOWN => "img", // Default extension
    };
    let file_name = format!("{}_{}.{}", file_stem, suffix, extension);
    output_path.set_file_name(file_name);

    // Save the image based on its type
    match file_type {
        FileType::JPG => {
            img.save_with_format(&output_path, image::ImageFormat::Jpeg)
                .map_err(|e| {
                    format!("Failed to save JPG image '{}': {}", output_path.display(), e)
                })?;
        }
        FileType::PNG => {
            img.save_with_format(&output_path, image::ImageFormat::Png)
                .map_err(|e| {
                    format!("Failed to save PNG image '{}': {}", output_path.display(), e)
                })?;
        }
        FileType::UNKNOWN => {
            img.save(&output_path).map_err(|e| {
                format!("Failed to save image '{}': {}", output_path.display(), e)
            })?;
        }
    }

    Ok(output_path)
}
