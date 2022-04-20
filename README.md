# RSA Generate Deterministic Keys

Generate RSA keys from a seed. 100% pure rust.

Uses a Hash-Based Message Authentication Code (HMAC) Deterministic Random Bit Generator (DRBG) with SHA2-256 (HmacDRBG-SHA2-256) to generate deterministic bytes from a seed, then use that seed to generate an RSA keypair.

## Demo App

The Rust library has been bundled into a demo app in the ./js folder to show how the library can be used.

## Test

To run test, run:

```cli
cargo test -- --nocapture
```
