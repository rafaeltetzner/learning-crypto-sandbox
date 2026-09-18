# Learning Crypto Sandbox

> A hands-on playground for learning cryptography — implemented, tested, benchmarked, and broken, one algorithm at a time.

[![Rust](https://img.shields.io/badge/language-Rust-orange)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-learning--in--progress-yellow)]()
[![License](https://img.shields.io/badge/license-see%20repo-lightgrey)]()

> ⚠️ **This is not a library.** These implementations are for learning, not production. For real-world cryptography, use audited libraries such as OpenSSL, *ring*, or RustCrypto.

---

## Table of Contents

- [Why this project?](#why-this-project)
- [Cryptographic security: three different guarantees](#cryptographic-security-three-different-guarantees)
- [Learning roadmap](#learning-roadmap)
- [Repository structure](#repository-structure)
- [Current progress](#current-progress)
- [References](#references)
- [Project status](#project-status)

---

## Why this project?

This project was born out of a [lecture by Sérgio Prado](https://e-labworks.com/) that I attended at my company, covering cryptographic algorithms in detail. It was only the second time I'd properly engaged with the topic — the first being during my undergraduate degree — and it made me realize I had taken a lot for granted: I had never actually *written* an implementation of these algorithms myself.

That gap turned into curiosity, and curiosity turned into this repository.

My goal is to understand — both mathematically and algorithmically — how and why algorithms like **AES**, **RSA**, **ECC**, **SHA-256**, and **ChaCha20** work, and why they exist in the first place. In my experience, these concepts click best through hands-on experimentation, trial and error, and actually breaking things — not just through lectures or videos.

**Rust** is the implementation language of choice, which means this project doubles as a practical way for me to learn the language.

Scope is intentionally flexible: some algorithms or concepts may be simplified, deferred, or skipped entirely if they aren't relevant enough or don't feel worth the time to implement.

---

## Cryptographic security: three different guarantees

A central theme of this project is that **"secure" doesn't mean the same thing across every cryptosystem**. There are three distinct guarantees worth telling apart.

### 1. Information-theoretic security

Some systems provide security that doesn't depend on an attacker's computational power at all. The classic example is the **One-Time Pad (OTP)**.

When correctly implemented — key is truly random, at least as long as the message, never reused, and kept secret — the ciphertext reveals *nothing* about the plaintext, even to an attacker with unlimited computing power. This property is called **perfect secrecy**.

The trade-off: secure distribution and management of a key as long as the message itself, which is rarely practical.

📄 [Information-Theoretic Security](docs/concepts/perfect-secrecy.md)

### 2. Computational security

Most modern cryptography doesn't offer perfect secrecy. Instead, it relies on **computational security**: breaking the system may be theoretically possible, but doing so should require resources that are infeasible under well-studied assumptions — such as the difficulty of integer factorization, discrete logarithms, or elliptic-curve discrete logarithms.

Algorithms explored here include:

```text
RSA · Diffie-Hellman · Elliptic-Curve Cryptography · AES · ChaCha20
```

It's worth internalizing the distinction between:

- *"Impossible to break"* — and
- *"Computationally infeasible to break"*

Modern cryptography aims for the second, not the first.

📄 [Computational Security](docs/concepts/computational-security.md)

### 3. Post-quantum cryptography

Quantum computing introduces a different threat model. Algorithms such as **Shor's algorithm** would break the assumptions behind factorization- and discrete-log-based public-key cryptography, if a sufficiently powerful quantum computer is ever built.

**Post-Quantum Cryptography (PQC)** responds to this: it runs on conventional computers today, but relies on mathematical problems for which no efficient quantum attack is currently known. The goal isn't "unbreakable" — it's resistance to a specific, more powerful class of attacker.

Planned areas of exploration:

```text
Post-Quantum Cryptography
        │
        ├── Key Encapsulation Mechanisms
        ├── Digital Signatures
        └── Quantum-resistant assumptions
```

📄 [Post-Quantum Cryptography](docs/concepts/post-quantum-security.md)

---

## Learning roadmap

The project moves from classical ciphers toward modern and post-quantum cryptography, with cryptanalysis studied alongside each stage rather than as an afterthought.

```text
                          CRYPTOGRAPHY
                               │
                 ┌─────────────┴──────────────┐
                 ▼                            ▼
          Classical Crypto          Cryptographic Concepts
                 │                            │
                 ▼                            ▼
              Caesar          Information-Theoretic Security
                 │                            │
                 ▼                            ▼
             Vigenère                   One-Time Pad
                 │                            │
                 ▼                            ▼
           Cryptanalysis             Computational Security
                 │                            │
                 ▼                            ▼
            Brute Force                   Modern Crypto
                 │                    ┌───────┴───────┐
                 ▼                    ▼               ▼
        Frequency Analysis        Symmetric       Public Key
                 │                 Crypto           Crypto
                 │                    │               │
                 │                    ▼               ▼
                 │                   AES             RSA
                 │                ChaCha20      Diffie-Hellman
                 │                    │              ECC
                 │                    │               │
                 └─────────────────┬──┴───────────────┘
                                   ▼
                            Quantum Threats
                                   │
                                   ▼
                          Post-Quantum Crypto
```

Each new topic is meant to build on the ones before it.

---

## Repository structure

```text
learning-crypto-sandbox/
│
├── README.md
│
├── assets/                    # diagrams, images
│
├── crates/
│   ├── classical/               # Caesar, Vigenère, Affine, ...
│   ├── symmetric/               # OTP, XOR, AES, ChaCha20
│   ├── asymmetric/              # RSA, Diffie-Hellman, ECC
│   ├── hashing/                 # SHA-256, SHA-3, HMAC
│   ├── attacks/                 # cryptanalysis implementations
│   ├── pqc/                     # post-quantum primitives
│   └── common/                  # shared utilities, traits, helpers
│       (each crate: README.md · Cargo.toml · src/ · tests/ · examples/ · benches/)
│
└── docs/
    ├── concepts/
    │   ├── perfect-secrecy.md
    │   ├── computational-security.md
    │   ├── post-quantum-security.md
    │   ├── modular-arithmetic.md
    │   └── entropy.md
    ├── attacks/
    │   ├── brute-force.md
    │   └── frequency-analysis.md
    └── roadmap.md
```

Each crate focuses on one area of cryptography; `docs/` holds the theory and concepts behind the code.

---

## Current progress

**Legend:** ✅ done · 🚧 in progress · ⬜ not started

### Classical cryptography

| Algorithm       | Implementation | Tests | Benchmark | Attacks |
| ---------------- | :-------------: | :---: | :-------: | :-----: |
| Caesar Cipher     |        ✅        |   ✅   |     ✅     |    🚧    |
| Vigenère Cipher   |        ⬜        |   ⬜   |     ⬜     |    ⬜    |
| Affine Cipher     |        ⬜        |   ⬜   |     ⬜     |    ⬜    |

### Symmetric cryptography

| Algorithm     | Implementation | Tests | Benchmark | Attacks |
| -------------- | :-------------: | :---: | :-------: | :-----: |
| XOR            |        ⬜        |   ⬜   |     ⬜     |    ⬜    |
| One-Time Pad   |        ⬜        |   ⬜   |     ⬜     |    ⬜    |
| AES            |        ⬜        |   ⬜   |     ⬜     |    ⬜    |
| ChaCha20       |        ⬜        |   ⬜   |     ⬜     |    ⬜    |

### Public-key cryptography

| Algorithm       | Implementation | Tests | Benchmark | Attacks / Weaknesses |
| ---------------- | :-------------: | :---: | :-------: | :-------------------: |
| RSA               |        ⬜        |   ⬜   |     ⬜     |          ⬜           |
| Diffie-Hellman    |        ⬜        |   ⬜   |     ⬜     |          ⬜           |
| ECC               |        ⬜        |   ⬜   |     ⬜     |          ⬜           |

### Hashing & authentication

| Primitive | Implementation | Tests | Benchmark |
| ---------- | :-------------: | :---: | :-------: |
| SHA-256    |        ⬜        |   ⬜   |     ⬜     |
| SHA-3      |        ⬜        |   ⬜   |     ⬜     |
| HMAC       |        ⬜        |   ⬜   |     ⬜     |

### Post-quantum cryptography

| Primitive | Implementation | Tests | Notes |
| ---------- | :-------------: | :---: | :----: |
| ML-KEM     |        ⬜        |   ⬜   |    ⬜    |
| ML-DSA     |        ⬜        |   ⬜   |    ⬜    |

> This roadmap is a learning plan, not a promise that every algorithm will be implemented from scratch.

---

## References

- *Serious Cryptography* — Jean-Philippe Aumasson
- *Cryptography Engineering* — Niels Ferguson, Bruce Schneier, Tadayoshi Kohno
- *Introduction to Modern Cryptography* — Jonathan Katz, Yehuda Lindell
- NIST cryptographic standards and publications
- Academic papers relevant to individual algorithms and attacks

Additional references specific to each lab are documented alongside the corresponding implementation.

---

## License

See the repository license for details.