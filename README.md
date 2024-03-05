# Transposition Ciphers

This repository contains implementations of transposition ciphers in both Rust and Python. Transposition ciphers are a type of symmetric key cipher where the positions of the characters in the plaintext are shifted according to a regular system, so that the ciphertext constitutes a permutation of the plaintext. This means that the cipher describes a way of rearranging the letters of the plaintext.

## Contents

- **Rust Implementation**: The Rust implementation is contained within the `main.rs` file. It includes functions for encrypting and decrypting using the Caesar, Rail Fence, and Columnar transposition ciphers.

- **Python Implementation**: The Python implementation is contained within the `Ciphers_in_python.py` file. It also includes functions for encrypting and decrypting using the Caesar, Rail Fence, and Columnar transposition ciphers.

## Features

- **Caesar Cipher**: A type of substitution cipher in which each letter in the plaintext is shifted a certain number of places down or up the alphabet.

- **Rail Fence Cipher**: A form of transposition cipher that gets its name from the way in which it's encoded. The message is written downwards on successive "rails" of an imaginary fence, then moving up when we get to the bottom rail.

- **Columnar Transposition Cipher**: A method of encryption by which the plaintext is written out in rows and then read off in columns.

## Usage

### Rust

To use the Rust implementation, compile the `main.rs` file and run the resulting binary. The `main` function demonstrates how to encrypt and decrypt messages using the Caesar, Rail Fence, and Columnar ciphers.

### Python

To use the Python implementation, run the `Ciphers_in_python.py` script. The script demonstrates how to encrypt and decrypt messages using the Caesar, Rail Fence, and Columnar ciphers.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any suggestions or improvements.
