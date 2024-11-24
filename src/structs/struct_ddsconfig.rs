// D:/Work/Projects/Modding/Skyrim/dds-edit/src/structs/struct_ddsconfig.rs

/// Configuration for DDS conversion.
///
/// Contains parameters that dictate how images are converted to DDS format.
pub struct DdsConvertConfig {
    /// Quality level for DDS compression (e.g., "Fast", "Best").
    pub quality: image_dds::Quality,

    /// Mipmap generation setting (e.g., "GeneratedAutomatic", "None").
    pub mipmaps: image_dds::Mipmaps,
}

impl DdsConvertConfig {
    /// Creates a new `DdsConvertConfig` with default values.
    ///
    /// # Returns
    ///
    /// * `Self` - An instance of `DdsConvertConfig` with preset configurations.
    pub fn new() -> Self {
        Self {
            quality: image_dds::Quality::Normal,
            mipmaps: image_dds::Mipmaps::GeneratedAutomatic,
        }
    }
}
