# Generate RSA Deterministic Keys

Generate RSA keys from a seed. 100% pure Rust.

⚠️ This completely works, unfortunately, it still takes way too long (122 seconds for a 4096 bit RSA key) ⚠️

-   [x] Pass in a seed (a-la [BIP39 seed phrase](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki))
-   [x] Get a Deterministic RSA Key in return
-   [x] Hash it using Sha2-256
-   [x] Base64-URL encode it (equiv to Arweave address)
-   [ ] Sign using the key
-   [ ] Verify Using the key

Uses a Hash-Based Message Authentication Code (HMAC) Deterministic Random Bit Generator (DRBG) with SHA2-256 (HmacDRBG-SHA2-256) to generate deterministic bytes from a seed, then use that seed to generate an RSA keypair.

## WHY?!

Because Arweave still uses only RSA keys until [#309](https://github.com/ArweaveTeam/arweave/pull/309) lands, so might as well use Rust + Wasm to generate and sign because RSA is slooooowwwwwww.

## Demo App

The Rust library has been bundled into a demo app in the `./sveltekit-rust-wasm` folder to show how the library can be packed into webassembly and used in a web app.

## Test

To run test, run:

```cli
cargo test -- --nocapture
```
