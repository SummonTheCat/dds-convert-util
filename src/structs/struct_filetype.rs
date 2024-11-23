// D:/Work/Projects/Modding/Skyrim/dds-edit/src/structs/struct_filetype.rs

/// Enumeration of supported image file types.
#[derive(Debug, Clone, Copy)]
pub enum FileType {
    /// Represents an unknown or unsupported file type.
    UNKNOWN,

    /// Represents JPEG file type.
    JPG,

    /// Represents PNG file type.
    PNG,
}

impl From<&str> for FileType {
    /// Converts a string slice to a `FileType` variant.
    ///
    /// # Arguments
    ///
    /// * `file_ext` - A string slice representing the file extension.
    ///
    /// # Returns
    ///
    /// * `FileType` - The corresponding `FileType` variant.
    fn from(file_ext: &str) -> Self {
        match file_ext.to_lowercase().as_str() {
            "jpg" | "jpeg" => FileType::JPG,
            "png" => FileType::PNG,
            _ => FileType::UNKNOWN,
        }
    }
}

impl std::fmt::Display for FileType {
    /// Formats the `FileType` for display purposes.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The result of the formatting operation.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileType::UNKNOWN => write!(f, "Unknown"),
            FileType::JPG => write!(f, "JPG"),
            FileType::PNG => write!(f, "PNG"),
        }
    }
}
