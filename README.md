# image-astc

[![Rust](https://github.com/Moetsuki/image-astc/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Moetsuki/image-astc/actions/workflows/rust.yml)

A decoder for ASTC (Adaptive Scalable Texture Compression) format images, built as an extension to the Rust `image`
crate.

## Overview

ASTC is a modern texture compression format that offers high-quality compression with flexible block sizes for various
types of image data. This crate provides functionality to decode ASTC-formatted images into standard formats that can be
used with the `image` crate's ecosystem.

## Usage

```rust
use image_astc::load_from_memory;

fn main() {
    let astc_data = std::fs::read("texture.astc").expect("Failed to read ASTC file");
    // Convert to u32 slice as needed
    let data_u32: &[u32] = bytemuck::cast_slice(&astc_data);
    
    // Load the image
    let image = load_from_memory(data_u32).expect("Failed to decode ASTC image");
    
    // Use the image as needed
    println!("Image dimensions: {}x{}", image.width(), image.height());
    let _pixels = img.to_rgba8().into_raw();
    
    // Save the image as a different format
    image.save("decoded.png").expect("Failed to save image");
}
```

## Format

This decoder expects a custom ASTC format with the following header:

- 12 byte signature: `A5 7C C7 5A 4F F4 5F 5F 4F F4 5F 5F`
- 2 byte version (little-endian)
- 8 byte width (little-endian)
- 8 byte height (little-endian)
- Followed by the compressed texture data

## License

This project is licensed under the MIT license.
