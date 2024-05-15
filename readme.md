# Bitcoin-rs

This is a simplified version of Bitcoin, implemented in Rust. The project is based on the book "Programming Bitcoin" by Jimmy Song.

## Project Status

Please note that this project is still under development and not yet ready for use.

## Features

- ECC (Elliptic Curve Cryptography) module for handling elliptic curve operations.
- SER (Serialization) module for handling serialization and deserialization of Bitcoin data structures.
- CORE module for the core Bitcoin functionality.

## Dependencies

This project uses the following dependencies:

- bnum: For handling big numbers.
- sha2: For SHA-256 hashing.
- hmac: For HMAC-SHA256 hashing.
- once_cell: For single assignment cells.
- ripemd: For RIPEMD-160 hashing.
- rand: For generating random numbers (used in dev environment).

## Contributing

Contributions are welcome. Please submit a pull request or create an issue to discuss the changes you want to make.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.