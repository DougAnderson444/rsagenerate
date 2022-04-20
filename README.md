# RSA Generate Deterministic Keys

Generate RSA keys from a seed. 100% pure rust.

Uses a Hash-Based Message Authentication Code (HMAC) Deterministic Random Bit Generator (DRBG) with SHA2-256 (HmacDRBG-SHA2-256) to generate deterministic bytes from a seed, then use that seed to generate an RSA keypair.

## Test

To run test, run:

```cli

cargo test -- --nocapture

```
