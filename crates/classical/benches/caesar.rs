// crates/classical/benches/caesar.rs

use classical::caesar::{decrypt, encrypt};
use common::random::random_text;
use criterion::{criterion_group, criterion_main, Criterion};

const INPUT_SIZE: usize = 1024 * 1024 * 10; // 10 MiB
const SHIFT: u8 = 11;

fn benchmark_encrypt(c: &mut Criterion) {
    let plaintext = random_text(INPUT_SIZE);

    c.bench_function("caesar/encrypt/10MiB", |b| {
        b.iter(|| encrypt(&plaintext,  SHIFT))
    });
}

fn benchmark_decrypt(c: &mut Criterion) {
    let plaintext = random_text(INPUT_SIZE);
    let ciphertext = encrypt(&plaintext, SHIFT);

    c.bench_function("caesar/decrypt/10MiB", |b| {
        b.iter(|| decrypt(&ciphertext, SHIFT))
    });
}

criterion_group!(benches, benchmark_encrypt, benchmark_decrypt);
criterion_main!(benches);
