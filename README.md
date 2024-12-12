# pqpg

**This project is under active development and is not presently safe for general use.**

`pqpg` (Post-Quantum Privacy Guard) is a command-line application providing high-level tools for managing post-quantum
cryptographic identities, signing and verifying messages, and exporting data securely. It aims to bring GPG-like
functionality to a post-quantum world, making advanced cryptography available for both technical and non-technical
users.

---

## Features

- **Post-Quantum Cryptography**: Secure signing and verification using SPHINCS+ (a quantum-resistant algorithm).
- **Identity Management**: Easily create and manage cryptographic identities with additional metadata like comments.
- **Key Management**: Generate key pairs, sign data, verify signatures, and control access to secret keys.

---

## Development

### Running Tests

To run the unit and integration tests:

```sh
cargo test
```

### Linting and Formatting

Use the following commands to ensure the code is properly formatted and adheres to best practices:

```sh
cargo fmt
cargo clippy
```

### Directory Structure

- **src/**: Contains the application's source code.

---

## Contributing

Contributions are welcome! If you would like to contribute:

1. Fork the repository.
2. Create a new feature branch (`git checkout -b feature-name`).
3. Add your changes, ensuring all tests pass.
4. Submit a pull request with a clear description of your changes.

---

## License

This project is licensed under the MIT License. View the license file [here](LICENSE).

---

## Acknowledgements

- Thanks to the `pqcrypto` crate for its quantum-resistant cryptographic algorithms.
- Inspired by the usability of GPG and similar cryptographic tools.
- Serialization and deserialization made easy by the `serde` crate.

---

## Contact

If you encounter any issues or have feature requests, feel free to open a ticket on
the [GitHub repository](https://github.com/gwo0d/pqpg).
