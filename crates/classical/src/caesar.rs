fn transform(byte: u8, shift: u8) -> u8 {
    match byte {
        b'a'..=b'z' => b'a' + (byte - b'a' + shift) % 26,
        b'A'..=b'Z' => b'A' + (byte - b'A' + shift) % 26,
        _ => byte,
    }
}

/// Encrypts plaintext using the Caesar cipher.
///
/// Each ASCII letter is shifted forward by shift positions while
/// preserving its case. Non-ASCII bytes and non-alphabetic ASCII bytes
/// are left unchanged.
///
/// The shift is reduced modulo 26, so a shift of 26 produces the
/// original plaintext.
///
/// # Examples
/// ```
/// use classical::caesar;
/// 
/// let plaintext = b"Hello, World!";
/// let shift = 3;
/// let ciphertext = caesar::encrypt(plaintext, shift);
/// assert_eq!(ciphertext, b"Khoor, Zruog!");
/// ```
pub fn encrypt(plaintext: &[u8], shift: u8) -> Vec<u8> {
    plaintext
        .iter()
        .map(|&byte| transform(byte, shift % 26))
        .collect()
}

/// Decrypts ciphertext encrypted with the Caesar cipher.
///
/// Each ASCII letter is shifted backward by shift positions while
/// preserving its case. Non-ASCII bytes and non-alphabetic ASCII bytes
/// are left unchanged.
///
/// The shift is reduced modulo 26, so a shift of 26 produces the
/// original ciphertext.
///
/// # Examples
/// ```
/// use classical::caesar;
/// 
/// let ciphertext = b"Khoor, Zruog!";
/// let shift = 3;
/// let plaintext = caesar::decrypt(ciphertext, shift);
/// assert_eq!(plaintext, b"Hello, World!");
/// ```
pub fn decrypt(ciphertext: &[u8], shift: u8) -> Vec<u8> {
    let shift = shift % 26;

    ciphertext
        .iter()
        .map(|&byte| transform(byte, (26 - shift) % 26))
        .collect()
}