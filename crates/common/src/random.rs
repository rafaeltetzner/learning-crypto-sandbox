use rand::{RngExt, distr::{Alphanumeric}};

pub fn random_bytes(size: usize) -> Vec<u8> {
    rand::rng()
        .random_iter::<u8>()
        .take(size)
        .collect()
}

pub fn random_text(size: usize) -> Vec<u8> {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .collect()
}