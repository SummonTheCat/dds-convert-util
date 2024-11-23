# DDS-Edit

DDS-Edit is a command-line tool for batch processing images, specifically designed for modding applications like Skyrim. It allows you to resize images to specified target widths and convert them to the DDS format, which is commonly used in game development for texture assets.

## Features

- **Batch Processing**: Recursively traverses directories to find and process image files.
- **Image Resizing**: Resizes images to multiple target widths while maintaining aspect ratio.
- **DDS Conversion**: Converts images to DDS format with configurable quality and mipmaps.
- **Concurrent Processing**: Utilizes multi-threading to speed up processing.
- **Configuration File Support**: Allows configurations to be loaded from a file or command-line arguments.

## Installation

To build and install DDS-Edit, you need to have Rust and Cargo installed.

```sh
cargo build --release
```

This will generate an executable in the `target/release` directory.

## Usage

```sh
dds-edit [OPTIONS] <INPUT_PATH>
```

### Command-Line Options

- `-o`, `--pathout <OUTPUT_PATH>`: Specify the output directory path.
- `-d`, `--maxdepth <MAX_DEPTH>`: Set the maximum directory traversal depth.
- `-t`, `--target-widths <WIDTHS>`: Comma-separated list of target widths for resizing images.
- `-n`, `--threads <THREADS>`: Number of threads to use for concurrent processing.
- `-c`, `--useconfig <CONFIG_FILE>`: Path to a configuration file.

### Examples

Resize all images in the `images/input` directory to widths of 1024 and 512 pixels, and save them to `images/output`:

```sh
dds-edit --pathout images/output --target-widths 1024,512 images/input
```

Use a configuration file:

```sh
dds-edit --useconfig config.toml
```

## Configuration File

You can specify a configuration file in TOML format to set default parameters.

### Example `config.toml`

```toml
path_in = "images/input"
path_out = "images/output"
max_depth = 5
target_sizes = [2048, 1024, 512]
threads = 8
```

## Detailed Documentation

For more detailed information on the usage, configuration, and implementation details, please refer to the [docs](docs) directory.

- [CLI Arguments](docs/cli_args.md)
- [Image Processing](docs/image_processing.md)
- [DDS Conversion](docs/dds_conversion.md)
- [Logging](docs/logging.md)
- [Configuration](docs/configuration.md)

## License

This project is licensed under the MIT License.

