# Sum Check Protocol

Work in progress... Right now, you will only see logs on the console. The prover and verifier implementation are still missing, as well as testing examples.

## Vision

The purpose of this application is to grasp knowledge about the considerations during the implementation of the Sum-Check Protocol while learning Rust. As of today, we have the definition of Multilinear Polynomials and Algebraic Operations on FieldElements. Since the main purpose is to learn, I am currently using u64 as integer type on Algebraic operations in FieldElement.

Note: This project is being developed with the assistance of ChatGPT, more specifically in the unit testing implementations, algorithmic reviews such as Horner's Method and Binary Exponentiation and some if not all Algebraic implementations corrections.

## Installation

I am using Rust because it is widely use in Cryptography area and I want to deepen my knowledge in it.

Make sure you have Rust Compiler and Cargo installed. Once installed, you may run the following command to setup the project:

```shell
cargo test && cargo run
```
or only:

```shell
cargo run
```

## Acknowledgments
This project was created as a learning exercise based on concepts from:
- *Proofs, Arguments and Zero-Knowledge* by Justin Thaler ([https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf])
  - Specifically, the Sum-Check Protocol described in Chapter 4 starting with simpler examples.
