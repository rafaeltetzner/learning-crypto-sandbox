//! Demonstrates applying a Caesar-like transformation to the pixel buffer
//! of an image, using the `image` crate to support arbitrary input formats
//! (PNG, JPEG, BMP, GIF, etc.).
//!
//! The classical Caesar cipher implemented by this crate operates on
//! alphabetic characters and therefore is not suitable for arbitrary binary
//! data such as an image. In this example, we intentionally do not use that
//! implementation. Instead, every channel byte in the decoded pixel buffer
//! is transformed directly using `wrapping_add` / `wrapping_sub`.
//!
//! The alpha channel (if any) is dropped: the image is normalized to RGB8
//! before the shift is applied, and the output is always re-encoded (here
//! as PNG) rather than reusing the original file's container format.
//! 
//! The generated output files are written to the `output` directory,
//! the overall png structure is preserved, and the image can be viewed
//! in any standard image viewer.

use image::{GenericImageView, ImageBuffer, Rgb};
use std::fs;
use std::io;
use std::path::Path;

// Shifts every channel byte by 128, which is equivalent to flipping the most significant bit.
const SHIFT: u8 = 128;

fn shift_buffer(buf: &[u8], shift: u8, encrypt: bool) -> Vec<u8> {
    buf.iter()
        .map(|&byte| {
            if encrypt {
                byte.wrapping_add(shift)
            } else {
                byte.wrapping_sub(shift)
            }
        })
        .collect()
}

fn save_rgb(
    path: &str,
    width: u32,
    height: u32,
    buf: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let img: ImageBuffer<Rgb<u8>, _> =
        ImageBuffer::from_raw(width, height, buf).ok_or("invalid buffer size for dimensions")?;
    img.save(path)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = "assets/sample.png";
    let encrypted_path = "output/sample_caesar_encrypted.png";
    let decrypted_path = "output/sample_caesar_decrypted.png";

    fs::create_dir_all("output")?;

    if !Path::new(input_path).exists() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            format!("input image not found: {input_path}"),
        )));
    }

    // Decode the image (format auto-detected from content/extension).
    let img = image::open(input_path)?;
    let (width, height) = img.dimensions();

    // Normalize to RGB8, dropping any alpha channel.
    let rgb = img.to_rgb8();
    let pixels: Vec<u8> = rgb.into_raw();

    // Encrypt every channel byte.
    let encrypted_pixels = shift_buffer(&pixels, SHIFT, true);
    save_rgb(encrypted_path, width, height, encrypted_pixels.clone())?;

    // Decrypt back.
    let decrypted_pixels = shift_buffer(&encrypted_pixels, SHIFT, false);
    save_rgb(decrypted_path, width, height, decrypted_pixels)?;

    println!("Original:   {input_path}");
    println!("Encrypted:  {encrypted_path}");
    println!("Decrypted:  {decrypted_path}");
    println!("Dimensions: {width}x{height}");
    println!("Pixel data: {} bytes", pixels.len());
    println!("Shift:      {SHIFT}");

    Ok(())
}