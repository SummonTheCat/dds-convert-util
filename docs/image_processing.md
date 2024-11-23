# Image Processing

This document describes how DDS-Edit processes images, including resizing and format conversion.

## Resizing Images

Images are resized to the specified target widths while maintaining the aspect ratio. The resizing is done using the `Triangle` filter, which offers a good balance between quality and performance.

### Target Widths

You can specify multiple target widths using the `--target-widths` option:

```sh
dds-edit --target-widths 2048,1024,512 ./images/input
```

If a target width is larger than the original image width or is zero, it will be skipped.

### Resizing Algorithm

The resizing is performed using the `resize` function from the `image` crate:

```rust
let resized_img = img.resize(target_width, new_height, FilterType::Triangle);
```

## Supported Image Formats

- JPEG (`.jpg`, `.jpeg`)
- PNG (`.png`)

Any files with extensions not matching these formats will be skipped or logged as unknown.

