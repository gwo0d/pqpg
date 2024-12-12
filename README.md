# pqpg

**This project is under active development and is not presently safe for general use.**

pqpg (Post-Quantum Privacy Guard) is a tool which provides high-level post-quantum cryptography implementations with support for signing and verification of messages using the SPHINCS+ algorithm. It is intended to provide GPG-like cryptography tools to both technical and non-technical users.

## Features

- Generate new key pairs for signing.
- Create and verify signatures.
- Encode and decode keys and signatures using Base64.
- Serialize and deserialize keys with optional inclusion of secret keys.

## Installation

Add `pqpg` to your `Cargo.toml`:

```toml
[dependencies]
pqpg = "0.1.0"
```

## Usage

### Generate a New Key Pair

```rust
use pqpg::SigningKey;

let signing_key = SigningKey::new();
let public_key = signing_key.get_public_key();
let secret_key = signing_key.get_secret_key().unwrap();
```

### Create a Signature

```rust
let message = b"test message";
let signature = signing_key.create_signature(message).unwrap();
```

### Verify a Signature

```rust
let is_valid = signing_key.verify_signature(message, signature);
assert!(is_valid);
```

### Deserialize Key from Public Key

```rust
let public_key = "<Base64 Encoded Public Key>";
let signing_key = SigningKey::new_from_public_key(public_key.to_string()).unwrap();
```

## Development

### Running Tests

To run the tests, use the following command:

```sh
cargo test
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any proposed changes or enhancements.

## License

This project is licensed under the MIT License.

## Acknowledgements

- The `pqcrypto` crate for providing the core cryptographic implementations.
- The `serde` crate for serialization support.

## Contact

For any questions or support, please open an issue on the [GitHub repository](https://github.com/gwo0d/pqpg).

---

This README provides an overview of the pqpg library, its features, installation instructions, usage examples, development guidelines, and contribution information. For more details, refer to the source code and documentation.
