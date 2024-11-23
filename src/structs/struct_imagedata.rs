// D:/Work/Projects/Modding/Skyrim/dds-edit/src/structs/struct_imagedata.rs

use std::path::{Path, PathBuf};
use super::struct_filetype::FileType;

/// Struct to hold image data.
///
/// Encapsulates all necessary metadata and paths required for processing an image.
#[derive(Clone)]
pub struct ImageData {
    /// Directory path of the input file.
    pub file_path_in: PathBuf,

    /// Directory path of the output file.
    pub file_path_out: PathBuf,

    /// File name without extension.
    pub file_name: String,

    /// File type (e.g., JPG, PNG).
    pub file_type: FileType,

    /// Number of color channels in the image (e.g., 3 for RGB, 4 for RGBA).
    pub channels: u8,
}

impl From<(PathBuf, PathBuf)> for ImageData {
    /// Constructs an `ImageData` instance from input and output paths.
    ///
    /// # Arguments
    ///
    /// * `paths` - A tuple containing the input and output `PathBuf`.
    ///
    /// # Returns
    ///
    /// * `Self` - An instance of `ImageData` populated with metadata.
    fn from(paths: (PathBuf, PathBuf)) -> Self {
        // Extract directory paths, defaulting to an empty path if unavailable.
        let file_path_in = paths.0.parent().unwrap_or_else(|| Path::new("")).to_path_buf();
        let file_path_out = paths.1.parent().unwrap_or_else(|| Path::new("")).to_path_buf();

        // Extract the file stem (name without extension).
        let file_name = paths.0.file_stem().unwrap_or_default().to_string_lossy().to_string();

        // Determine the file type based on the file extension.
        let filetype_string = paths.0.extension().unwrap_or_default().to_string_lossy();
        let file_type = FileType::from(filetype_string.as_ref());

        ImageData {
            file_path_in,
            file_path_out,
            file_name,
            file_type,
            channels: 0, // Default to 0; will be updated after image loading.
        }
    }
}

impl std::fmt::Display for ImageData {
    /// Formats the `ImageData` for display purposes.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The result of the formatting operation.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "ImageData {{\n  file_path_in: {:?},\n  file_path_out: {:?},\n  file_name: {},\n  file_type: {},\n  channels: {}\n}}",
            self.file_path_in, self.file_path_out, self.file_name, self.file_type, self.channels
        )
    }
}
