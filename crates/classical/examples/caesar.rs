//! Demonstrates applying a Caesar-like transformation to the pixel data of
//! a BMP image.
//!
//! The classical Caesar cipher implemented by this crate operates on
//! alphabetic characters and therefore is not suitable for arbitrary binary
//! data such as an image. In this example, we intentionally do not use that
//! implementation. Instead, every pixel byte is transformed directly using
//! `wrapping_add` and `wrapping_sub`.
//!
//! The BMP header is preserved so that the encrypted file remains a valid
//! BMP image. Only the pixel data is transformed using wrapping arithmetic.
//!
//! This is intended as a demonstration of applying the idea of a Caesar
//! shift to arbitrary bytes, rather than an implementation of the classical
//! Caesar cipher.

use std::fs;
use std::io;

const SHIFT: u8 = 42;

fn main() -> io::Result<()> {
    let input_path = "assets/tux.bmp";
    let encrypted_path = "output/tux_caesar_encrypted.bmp";
    let decrypted_path = "output/tux_caesar_decrypted.bmp";

    fs::create_dir_all("output")?;

    // Read the entire BMP file.
    let image = fs::read(input_path)?;

    // A BMP file must contain at least the 14-byte file header.
    if image.len() < 14 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "BMP file is too small",
        ));
    }

    // Check the BMP signature.
    if &image[0..2] != b"BM" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "File is not a BMP image",
        ));
    }

    // Bytes 10..14 contain the offset to the pixel data.
    let pixel_offset = u32::from_le_bytes([
        image[10],
        image[11],
        image[12],
        image[13],
    ]) as usize;

    if pixel_offset > image.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "BMP pixel offset is outside the file",
        ));
    }

    // Keep the BMP header unchanged and transform only the pixel data.
    let (header, pixels) = image.split_at(pixel_offset);

    // Encrypt each pixel byte using wrapping addition.
    let encrypted_pixels: Vec<u8> = pixels
        .iter()
        .map(|&byte| byte.wrapping_add(SHIFT))
        .collect();

    // Reconstruct the encrypted BMP.
    let mut encrypted_image = Vec::with_capacity(image.len());
    encrypted_image.extend_from_slice(header);
    encrypted_image.extend_from_slice(&encrypted_pixels);

    fs::write(encrypted_path, encrypted_image)?;

    // Decrypt each pixel byte using wrapping subtraction.
    let decrypted_pixels: Vec<u8> = encrypted_pixels
        .iter()
        .map(|&byte| byte.wrapping_sub(SHIFT))
        .collect();

    // Reconstruct the decrypted BMP.
    let mut decrypted_image = Vec::with_capacity(image.len());
    decrypted_image.extend_from_slice(header);
    decrypted_image.extend_from_slice(&decrypted_pixels);

    fs::write(decrypted_path, decrypted_image)?;

    println!("Original:   {input_path}");
    println!("Encrypted:  {encrypted_path}");
    println!("Decrypted:  {decrypted_path}");
    println!("Pixel data: {} bytes", pixels.len());
    println!("Shift:      {SHIFT}");

    Ok(())
}
