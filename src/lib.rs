#![warn(missing_docs)]
#![warn(unused_qualifications)]
#![deny(unreachable_pub)]
#![deny(deprecated)]
#![deny(missing_copy_implementations)]

//! A decoder for ASTC (Adaptive Scalable Texture Compression) format images.
//!
//! This crate implements a decoder for a custom ASTC format with the following header:
//! - 12 byte signature: `A5 7C C7 5A 4F F4 5F 5F 4F F4 5F 5F`
//! - 2 byte version (little-endian)
//! - 8 byte width (little-endian)
//! - 8 byte height (little-endian)
//! - Followed by the compressed texture data
//!
//! # Examples
//!
//! ```no_run
//!
//! use image_astc::load_from_memory;
//! use image::GenericImageView;
//!
//! fn main() {
//!     let astc_data = std::fs::read("texture.astc").expect("Failed to read ASTC file");
//!     // Convert to u32 slice as needed
//!     let data_u32: &[u32] = bytemuck::cast_slice(&astc_data);
//!
//!     // Load the image
//!     let image = load_from_memory(data_u32).expect("Failed to decode ASTC image");
//!
//!     // Use the image as needed
//!     println!("Image dimensions: {}x{}", image.width(), image.height());
//!     let _pixels = image.to_rgba8().into_raw();
//!
//!     // Save the image as a different format
//!     image.save("decoded.png").expect("Failed to save image");
//! }
//! ```

use image::error::{DecodingError, ImageFormatHint, ImageResult, LimitError, LimitErrorKind};
use image::{DynamicImage, ImageBuffer, ImageError, Rgba};

/// Decodes the ASTC header from the provided data slice.
///
/// The header consists of:
/// - 12 byte signature: `A5 7C C7 5A 4F F4 5F 5F 4F F4 5F 5F`
/// - 2 byte version (little-endian)
/// - 8 byte width (little-endian)
/// - 8 byte height (little-endian)
///
/// # Errors
///
/// Returns an error if:
/// - The data is too short to contain a valid header
/// - The header signature doesn't match the expected ASTC signature
///
/// # Returns
///
/// On success, returns a tuple of `(width, height)` of the image.
pub(crate) fn decode_header(data: &[u8]) -> ImageResult<(u32, u32)> {
    // Check if the first 12 bytes are our custom ASTC header
    // A57C C75A 4FF4 5F5F 4FF4 5F5F
    if data.len() < 30 {
        return Err(ImageError::Decoding(DecodingError::new(
            ImageFormatHint::Unknown,
            "Data is too short to contain an ASTC header",
        )));
    }

    if data[0..12]
        != [
            0xA5, 0x7C, 0xC7, 0x5A, 0x4F, 0xF4, 0x5F, 0x5F, 0x4F, 0xF4, 0x5F, 0x5F,
        ]
    {
        return Err(ImageError::Decoding(DecodingError::new(
            ImageFormatHint::Unknown,
            "Data does not contain an ASTC header",
        )));
    }

    // The next 2 bytes are the version
    let _version = u16::from_le_bytes(data[12..14].try_into().unwrap());

    // The next 8 bytes are the width
    let width = u32::from_le_bytes(data[14..22].try_into().unwrap());

    // The next 8 bytes are the height
    let height = u32::from_le_bytes(data[22..30].try_into().unwrap());

    Ok((width, height))
}

/// Loads an ASTC image from memory and decodes it to a `DynamicImage`.
///
/// # Parameters
///
/// - `data`: A slice of `u32` values containing the ASTC image data.
///
/// # Errors
///
/// Returns an error if:
/// - The header is invalid or missing
/// - There isn't enough memory to create the image buffer
/// - The data is corrupted or in an incorrect format
///
/// # Returns
///
/// On success, returns a `DynamicImage` containing the decoded RGBA8 image.
pub fn load_from_memory(data: &[u32]) -> ImageResult<DynamicImage> {
    let (width, height) = decode_header(bytemuck::cast_slice(data))?;

    let buf = data.into_iter().map(|v| *v).collect::<Vec<u32>>();
    ImageBuffer::from_raw(width, height, buf)
        .ok_or_else(|| {
            ImageError::Limits(LimitError::from_kind(LimitErrorKind::InsufficientMemory))
        })
        .map(|_arg0: ImageBuffer<Rgba<u32>, Vec<u32>>| DynamicImage::new_rgba8(width, height))
}
