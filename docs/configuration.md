# Configuration

DDS-Edit can be configured using command-line arguments or a configuration file in TOML format. Command-line arguments override settings in the configuration file.

## Configuration File

The configuration file allows you to set default parameters so you don't have to specify them every time you run DDS-Edit.

### Example `config.toml`

```toml
path_in = "images/input"
path_out = "images/output"
max_depth = 5
target_sizes = [2048, 1024, 512]
threads = 8
```

## Configuration Parameters

- `path_in`: Input directory containing images to process.
- `path_out`: Output directory where processed images will be saved.
- `max_depth`: Maximum directory traversal depth.
- `target_sizes`: List of target widths for resizing images.
- `threads`: Number of threads for concurrent processing.

## Loading Configuration

To use a configuration file, specify it with the `--useconfig` option:

```sh
dds-edit --useconfig config.toml
```

## Merging Configurations

When both a configuration file and command-line arguments are provided, DDS-Edit merges them, giving precedence to command-line arguments.

## Default Values

If certain parameters are not specified, DDS-Edit uses the following default values:

- `path_out`: `./images/output`
- `max_depth`: Unlimited (`usize::MAX`)
- `target_sizes`: `[2048]`
- `threads`: `4`

## Notes

- The `path_in` parameter must be specified either in the configuration file or as a command-line argument.
- Future versions may include more configurable options such as DDS quality and mipmap settings.

