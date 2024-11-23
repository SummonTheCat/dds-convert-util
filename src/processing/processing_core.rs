// D:/Work/Projects/Modding/Skyrim/dds-edit/src/processing/processing_core.rs

use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    io::io_pathtree::mirror_path,
    structs::{struct_config::Config, struct_imagedata::ImageData},
};

use super::processing_img::process_img;

/// Processes a list of image paths based on the provided configuration.
///
/// This function distributes image processing tasks asynchronously.
///
/// # Arguments
///
/// * `filtered_paths` - Slice of `PathBuf` pointing to image files to process.
/// * `config` - Reference to the `Config` struct containing configuration parameters.
///
/// # Returns
///
/// * `Ok(())` if processing succeeds.
/// * `Err(String)` with an error message if processing fails.
pub async fn process_files(
    filtered_paths: &[PathBuf],
    config: &Config,
) -> Result<(), String> {
    // Convert input and output paths to Path objects.
    let path_in = Path::new(&config.path_in);
    let path_out = Path::new(&config.path_out);

    // Wrap filtered_paths in an Arc to allow safe sharing across tasks.
    let paths = Arc::new(filtered_paths.to_owned());

    // Determine the total number of images to process.
    let total_images = paths.len();
    let concurrency_limit = config.threads;

    // Keep start time
    let start_time = std::time::Instant::now();

    // Create a semaphore to limit concurrency.
    let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency_limit));

    // Vector to hold the join handles of spawned tasks.
    let mut handles = Vec::new();

    for idx in 0..total_images {
        let paths_clone = Arc::clone(&paths);
        let config_clone = config.clone();
        let path_in = path_in.to_path_buf();
        let path_out = path_out.to_path_buf();
        let semaphore_clone = Arc::clone(&semaphore);

        // Acquire a permit before spawning the task.
        let permit = semaphore_clone.acquire_owned().await.unwrap();

        // Spawn an asynchronous task.
        let handle = tokio::spawn(async move {
            // Ensure the permit is held for the duration of the task.
            let _permit = permit;

            let input_path = &paths_clone[idx];

            // Compute the mirrored output path.
            let output_path = match mirror_path(&path_in, &path_out, input_path) {
                Ok(op) => op,
                Err(e) => {
                    eprintln!(
                        "Error mirroring path '{}': {}",
                        input_path.display(),
                        e
                    );
                    return;
                }
            };

            // Ensure that the parent directory exists in the output path.
            if let Some(parent) = output_path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!(
                        "Failed to create directory '{}': {}",
                        parent.display(),
                        e
                    );
                    return;
                }
            }

            // Create an ImageData instance from the input and output paths.
            let image_data =
                ImageData::from((input_path.clone(), output_path.clone()));
            // Process the image (resizing and DDS conversion).
            process_img(image_data, &config_clone).await;
        });

        // Store the task handle.
        handles.push(handle);
    }

    // Wait for all tasks to finish processing.
    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Task panicked: {:?}", e);
        }
    }

    // Calculate the total processing time.
    let elapsed_time = start_time.elapsed().as_secs_f64();
    println!("--------------------------");
    println!("Processing completed in {:.2} seconds.", elapsed_time);
    println!("--------------------------");

    Ok(())
}
