# DDS Conversion

DDS-Edit converts images to the DDS format after resizing. This document explains how the conversion works and the configurations involved.

## DDS Format

DDS (DirectDraw Surface) is a container format for storing compressed graphical textures and cubic environment maps. It is commonly used in game development.

## Conversion Process

After resizing, images are converted to DDS format using the `image-dds` crate.

### Determining Image Format

The appropriate DDS format is selected based on the number of color channels in the image:

- **3 channels (RGB)**: Uses `BC1RgbaUnormSrgb`.
- **4 channels (RGBA)**: Uses `BC7RgbaUnormSrgb`.

### Example Code

```rust
let image_format = match channels {
    3 => ImageFormat::BC1RgbaUnormSrgb,
    4 => ImageFormat::BC7RgbaUnormSrgb,
    _ => ImageFormat::BC1RgbaUnormSrgb,
};
```

### DDS Conversion Function

The conversion is performed using:

```rust
let dds = image_dds::dds_from_image(&image, image_format, quality, mipmaps)?;
```

## Configuration Options

Currently, the DDS conversion uses default settings:

- **Quality**: `Fast`
- **Mipmaps**: `GeneratedAutomatic`

Future versions may allow customization of these settings via the configuration file or command-line arguments.

