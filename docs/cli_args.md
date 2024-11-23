# CLI Arguments

DDS-Edit accepts various command-line arguments to customize its behavior. This document provides detailed information on how to use these arguments.

## Usage

```sh
dds-edit [OPTIONS] <INPUT_PATH>
```

## Options

- `-o`, `--pathout <OUTPUT_PATH>`: Specify the output directory path. If not provided, defaults to `./images/output`.

- `-d`, `--maxdepth <MAX_DEPTH>`: Set the maximum directory traversal depth. Default is unlimited.

- `-t`, `--target-widths <WIDTHS>`: Comma-separated list of target widths for resizing images. Defaults to `[2048]`.

- `-n`, `--threads <THREADS>`: Number of threads to use for concurrent processing. Defaults to `4`.

- `-c`, `--useconfig <CONFIG_FILE>`: Path to a configuration file in TOML format.

## Examples

Resize images to widths of 1024 and 512 pixels:

```sh
dds-edit --target-widths 1024,512 ./images/input
```

Specify maximum directory depth:

```sh
dds-edit --maxdepth 5 ./images/input
```

Use a configuration file:

```sh
dds-edit --useconfig config.toml
```

## Notes

- If a configuration file is specified with `--useconfig`, any command-line arguments will override the settings in the configuration file.

- The `<INPUT_PATH>` is required unless a configuration file is used that specifies `path_in`.

