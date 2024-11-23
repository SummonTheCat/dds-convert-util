// D:/Work/Projects/Modding/Skyrim/dds-edit/src/processing/processing_img.rs

use crate::{
    io::io_out::save_image,
    logging::{LogQueue, LogType},
    processing::processing_dds::convert_to_dds,
    structs::{
        struct_config::Config,
        struct_filetype::FileType,
        struct_imagedata::ImageData,
    },
};
use image::{self, imageops::FilterType, GenericImageView};
use colored::Color;

/// Processes an individual image based on its file type.
///
/// # Arguments
///
/// * `image_data` - The `ImageData` instance containing image information.
/// * `_config` - Reference to the `Config` struct containing configuration parameters.
pub async fn process_img(image_data: ImageData, _config: &Config) {
    // Determine the processing function based on the image type.
    match image_data.file_type {
        FileType::JPG => process_image_base(image_data, _config).await,
        FileType::PNG => process_image_base(image_data, _config).await,
        FileType::UNKNOWN => process_image_unknown(image_data, _config).await,
    }
}

/// Processes images with known file types (JPG and PNG).
///
/// This function handles resizing the image to multiple target widths and converting each resized image to DDS format.
///
/// # Arguments
///
/// * `image_data` - The `ImageData` instance containing image information.
/// * `config` - Reference to the `Config` struct containing configuration parameters.
pub async fn process_image_base(
    mut image_data: ImageData,
    config: &Config,
) {
    let log_queue = LogQueue::new(); // Create a new log queue.

    // Log the start of processing for this image.
    log_queue.enqueue(
        LogType::Println,
        "--------------------------".to_string(),
        Color::White,
    );
    log_queue.enqueue(
        LogType::Print,
        "[Processing Image: ".to_string(),
        Color::White,
    );
    log_queue.enqueue(
        LogType::Println,
        format!("{}]", image_data.file_name),
        Color::White,
    );

    // Log "Input Path: " in white and the actual path in bright cyan.
    log_queue.enqueue(
        LogType::Print,
        "Input Path: ".to_string(),
        Color::White,
    );
    log_queue.enqueue(
        LogType::Println,
        format!("{}", image_data.file_path_in.display()),
        Color::BrightCyan,
    );

    // Log "Output Path: " in white and the actual path in bright cyan.
    log_queue.enqueue(
        LogType::Print,
        "Output Path: ".to_string(),
        Color::White,
    );
    log_queue.enqueue(
        LogType::Println,
        format!("{}", image_data.file_path_out.display()),
        Color::BrightCyan,
    );

    // Construct the full input file path.
    let input_file_path = image_data.file_path_in.join(format!(
        "{}.{}",
        image_data.file_name,
        image_data.file_type.to_string().to_lowercase()
    ));

    // Clone input_file_path before moving into closure
    let input_file_path_clone = input_file_path.clone();

    // Attempt to open the image file.
    let img = match tokio::task::spawn_blocking(move || image::open(&input_file_path_clone)).await {
        Ok(img_result) => match img_result {
            Ok(img) => img,
            Err(e) => {
                // Handle ImageError
                log_queue.enqueue(
                    LogType::Println,
                    format!(
                        "Failed to open image '{}': {}",
                        input_file_path.display(),
                        e
                    ),
                    Color::Red,
                );
                log_queue.process_queue();
                return;
            }
        },
        Err(e) => {
            // Handle JoinError
            log_queue.enqueue(
                LogType::Println,
                format!(
                    "Task failed to run to completion for '{}': {}",
                    input_file_path.display(),
                    e
                ),
                Color::Red,
            );
            log_queue.process_queue();
            return;
        }
    };

    // Retrieve image dimensions and channel count.
    let (width, height) = img.dimensions();
    let channels = img.color().channel_count();
    image_data.channels = channels; // Update the channel count in ImageData.

    // Log image dimensions.
    log_queue.enqueue(
        LogType::Print,
        "Image Dimensions: ".to_string(),
        Color::White,
    );
    log_queue.enqueue(
        LogType::Println,
        format!("Width = {}, Height = {}", width, height),
        Color::BrightCyan,
    );

    // Iterate over each target width specified in the configuration.
    for &target_width in &config.target_sizes {
        log_queue.enqueue(
            LogType::Println,
            format!("[Started Processing for Width: {}]", target_width),
            Color::BrightBlue,
        );

        // Skip invalid target widths.
        if target_width == 0 || target_width > width {
            log_queue.enqueue(
                LogType::Println,
                format!(
                    "Skipping target width {} for image '{}'.",
                    target_width, image_data.file_name
                ),
                Color::Yellow,
            );
            continue;
        }

        // Calculate the new height to maintain the aspect ratio.
        let new_height =
            ((target_width as f32 / width as f32) * height as f32) as u32;

        // Clone img before moving into closure
        let img_clone = img.clone();

        // Resize the image asynchronously.
        let resized_img = match tokio::task::spawn_blocking(move || {
            img_clone.resize(target_width, new_height, FilterType::Triangle)
        })
        .await
        {
            Ok(resized_result) => resized_result,
            Err(e) => {
                log_queue.enqueue(
                    LogType::Println,
                    format!("Failed to resize image '{}': {}", image_data.file_name, e),
                    Color::Red,
                );
                continue;
            }
        };

        // Prepare variables for output path and file type
        let file_type_string = image_data.file_type.to_string().to_lowercase();
        let output_path = image_data.file_path_out.join(format!(
            "{}.{}",
            image_data.file_name,
            file_type_string
        ));

        // Prepare variables to move into the closure
        let resized_img_clone = resized_img.clone();
        let output_path_clone = output_path.clone();
        let target_width_string = target_width.to_string();
        let file_type = image_data.file_type; // FileType is now Copy

        // Save the resized image.
        let save_result = match tokio::task::spawn_blocking(move || {
            save_image(
                &resized_img_clone,
                &output_path_clone,
                &target_width_string,
                file_type, // Pass by value
            )
        })
        .await
        {
            Ok(save_result) => save_result,
            Err(e) => {
                log_queue.enqueue(
                    LogType::Println,
                    format!(
                        "Failed to spawn blocking task for saving image '{}': {}",
                        image_data.file_name, e
                    ),
                    Color::Red,
                );
                continue;
            }
        };

        match save_result {
            Ok(saved_path) => {
                log_queue.enqueue(
                    LogType::Print,
                    "Saved resized image: ".to_string(),
                    Color::White,
                );
                log_queue.enqueue(
                    LogType::Println,
                    format!("'{}'", saved_path.display()),
                    Color::Green,
                );

                // Define the output path for the DDS conversion.
                let output_path_dds = image_data.file_path_out.join(format!(
                    "{}_{}.{}",
                    image_data.file_name,
                    target_width,
                    file_type_string
                ));

                // Clone output_path_dds and image_data for moving into closure
                let output_path_dds_clone = output_path_dds.clone();
                let image_data_clone = image_data.clone();

                // Convert the resized image to DDS format.
                let convert_result =
                    convert_to_dds(&output_path_dds_clone, &image_data_clone).await;
                match convert_result {
                    Ok(dds_path) => {
                        log_queue.enqueue(
                            LogType::Print,
                            "Converted to DDS: ".to_string(),
                            Color::White,
                        );
                        log_queue.enqueue(
                            LogType::Println,
                            format!("'{}'", dds_path.display()),
                            Color::Green,
                        );
                    }
                    Err(e) => {
                        log_queue.enqueue(
                            LogType::Println,
                            format!("Failed to convert image to DDS: {}", e),
                            Color::Red,
                        );
                    }
                }
            }
            Err(e) => {
                // Log the error if saving the resized image fails.
                log_queue.enqueue(
                    LogType::Println,
                    format!(
                        "Failed to save resized image for '{}': {}",
                        image_data.file_name, e
                    ),
                    Color::Red,
                );
            }
        }
    }

    // Process and print all accumulated logs.
    log_queue.process_queue();
}

/// Processes images with unknown file types.
///
/// This function logs relevant information without performing any processing.
///
/// # Arguments
///
/// * `image_data` - The `ImageData` instance containing image information.
/// * `_config` - Reference to the `Config` struct containing configuration parameters.
pub async fn process_image_unknown(
    image_data: ImageData,
    _config: &Config,
) {
    let log_queue = LogQueue::new(); // Create a new log queue.

    // Log details about the image with an unknown file type.
    log_queue.enqueue(
        LogType::Println,
        "[Processing Image: Unknown]".to_string(),
        Color::White,
    );
    log_queue.enqueue(
        LogType::Print,
        "Input Path: ".to_string(),
        Color::White,
    );
    log_queue.enqueue(
        LogType::Println,
        format!("{}", image_data.file_path_in.display()),
        Color::BrightCyan,
    );
    log_queue.enqueue(
        LogType::Print,
        "Output Path: ".to_string(),
        Color::White,
    );
    log_queue.enqueue(
        LogType::Println,
        format!("{}", image_data.file_path_out.display()),
        Color::BrightCyan,
    );
    log_queue.enqueue(
        LogType::Println,
        "Image Type: Unknown".to_string(),
        Color::BrightRed,
    );

    // Process and print all accumulated logs.
    log_queue.process_queue();
}
