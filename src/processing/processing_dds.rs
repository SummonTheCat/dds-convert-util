// D:/Work/Projects/Modding/Skyrim/dds-edit/src/processing/processing_dds.rs

use std::path::{Path, PathBuf};
use image_dds::ImageFormat;
use image; // Ensure the `image` crate is in scope.

use crate::structs::{
    struct_ddsconfig::DdsConvertConfig,
    struct_imagedata::ImageData,
};

/// Maps the number of image channels to the appropriate ImageFormat.
pub fn map_channels_format(channels: u8) -> ImageFormat {
    match channels {
        3 => ImageFormat::BC1RgbaUnormSrgb, // RGB without alpha
        4 => ImageFormat::BC7RgbaUnormSrgb, // RGBA with full alpha
        _ => ImageFormat::BC1RgbaUnormSrgb, // Default to BC1
    }
}

/// Converts an image to a DDS format based on the provided configuration.
///
/// # Arguments
///
/// * `image_path` - Path to the input image.
/// * `image_data` - Reference to the `ImageData` struct containing image metadata.
///
/// # Returns
///
/// * `Ok(PathBuf)` with the path to the saved DDS file.
/// * `Err(String)` with an error message if conversion fails.
pub async fn convert_to_dds(
    image_path: &Path,
    image_data: &ImageData,
) -> Result<PathBuf, String> {
    // Extract the file stem (name without extension).
    let file_name = image_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| {
            format!("Invalid file name for path: {:?}", image_path)
        })?
        .to_string();

    // Clone or convert references to owned data
    let image_path_owned = image_path.to_owned(); // Convert to PathBuf
    let file_path_out = image_data.file_path_out.clone();
    let channels = image_data.channels;

    // Perform the conversion in a blocking task.
    let result = tokio::task::spawn_blocking(move || {
        // Use image_path_owned inside the closure
        let image = image::open(&image_path_owned)
            .map_err(|e| {
                format!(
                    "Failed to open image '{}': {}",
                    image_path_owned.display(),
                    e
                )
            })?
            .to_rgba32f();

        // Apply gamma correction to the image.
        let image = apply_gamma_correction(&image);
        
        // Create default DDS conversion configuration.
        let config = DdsConvertConfig::new();

        // Determine the appropriate ImageFormat based on channel count.
        let image_format = map_channels_format(channels);

        // Convert the image to DDS format.
        let dds_result = image_dds::dds_from_imagef32(
            &image,
            image_format,
            config.quality,
            config.mipmaps,
        );

        match dds_result {
            Ok(dds) => {
                // Define the output DDS file path.
                let dds_path = file_path_out.join(format!("{}.dds", file_name));

                // Create a buffered writer for the DDS file.
                let mut writer = std::io::BufWriter::new(
                    std::fs::File::create(&dds_path).map_err(|e| {
                        format!(
                            "Failed to create DDS file '{}': {}",
                            dds_path.display(),
                            e
                        )
                    })?,
                );

                // Write the DDS data to the file.
                dds.write(&mut writer)
                    .map_err(|e| {
                        format!(
                            "Failed to write DDS file '{}': {}",
                            dds_path.display(),
                            e
                        )
                    })?;

                // Return the path to the DDS file.
                Ok(dds_path)
            }
            Err(e) => {
                // Return an error if the DDS conversion fails.
                Err(format!("Failed to convert image to DDS: {}", e))
            }
        }
    })
    .await
    .map_err(|e| format!("Failed to spawn blocking task: {}", e))??;

    Ok(result)
}


fn apply_gamma_correction(image: &image::Rgba32FImage) -> image::Rgba32FImage {
    image::Rgba32FImage::from_fn(image.width(), image.height(), |x, y| {
        let pixel = image.get_pixel(x, y);
        image::Rgba([
            pixel[0].powf(1.0 / 2.2),
            pixel[1].powf(1.0 / 2.2),
            pixel[2].powf(1.0 / 2.2),
            pixel[3],
        ])
    })
}
