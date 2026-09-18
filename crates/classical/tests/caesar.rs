use classical::caesar::{decrypt, encrypt};
use common::random::random_text;

#[test]
fn encrypts_lowercase_letters() {
	assert_eq!(encrypt(b"attackatdawn", 3), b"dwwdfndwgdzq");
}

#[test]
fn encrypts_uppercase_letters() {
	assert_eq!(encrypt(b"HELLO", 3), b"KHOOR");
}

#[test]
fn preserves_non_alphabetic_characters() {
	assert_eq!(encrypt(b"Hello, World!", 3), b"Khoor, Zruog!");
}

#[test]
fn wraps_at_the_end_of_the_alphabet() {
	assert_eq!(encrypt(b"xyz XYZ", 3), b"abc ABC");
}

#[test]
fn decrypt_reverses_encryption() {
	let plaintext = "Meet me at 10:30, friend!";
	let ciphertext = encrypt(plaintext.as_bytes(), 11);

	assert_eq!(decrypt(&ciphertext, 11), plaintext.as_bytes());
}

#[test]
fn shifts_larger_than_one_alphabet_length() {
	assert_eq!(encrypt(b"abc", 55), b"def");
}

#[test]
fn decrypt_reverses_encryption_with_large_text() {
    let plaintext_bytes = random_text(1024 * 1024);


    let ciphertext = encrypt(plaintext_bytes.as_slice(), 11);

    assert_eq!(decrypt(&ciphertext, 11), plaintext_bytes.as_slice());
}