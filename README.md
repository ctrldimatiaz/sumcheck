# Sum-Check Protocol

## Protocol Description 

Central mathematic problems is computing the sum of:

H := ∑b1∈{0,1}∑ b2∈{0,1} ...∑ bv∈{0,1} (g(b1,...,bv)) 

The prover initially claims a value C₁ equal to H. In each subsequent round, the prover sends a univariate polynomial that is claimed to be the appropriate partial sum of the original polynomial. The verifier does not check that the polynomial reduced by the prover is derived from the original polynomial g. Instead, it checks consistency in the response between rounds through random challenges that make it difficult (if not statistical negligible) for a dishonest prover to maintain consistency with a false claim.

Prover &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Verifier

C₁ = claimed H <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;───────────────────────><br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<─────────────────────── Asks to prove it<br>
        
g₁(X₁) &nbsp;───────────────────────> check: C₁ = g₁(0) + g₁(1)<br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<─────────────────────── choose r₁<br>

g₂(X₂) &nbsp;───────────────────────> check: g₁(r₁) = g₂(0) + g₂(1)<br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<─────────────────────── choose r₂<br>
...<br>

gᵥ(Xᵥ) &nbsp;───────────────────────> check: gᵥ₋₁(rᵥ₋₁) = gᵥ(0) + gᵥ(1)<br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<─────────────────────── choose rᵥ<br>
        
─────────────> final verifier check: gᵥ(rᵥ) = g(r₁,...,rᵥ)

Generally, the polynomail gj(X) is constructed as:
gj​(Xj​)=xj+1​,…,xv​∈{0,1}∑​g(r1​,…,rj−1​,Xj​,xj+1​,…,xv​).

## Mathematic concepts vs implementation

`/field` - Representation of field elements and its algebraic operations

- Finite Field: Fp where p is prime and denote the set of integers modulo p. Used through `FieldElement<P>` for a chosen prime P under /field folder. Because p is prime, the integers modulo p form a field Fp. Addition, subtraction, and multiplication are closed operations, and every nonzero element has a multiplicative inverse, allowing division by nonzero elements.

`/polynomials` - Representation of the polynomials needed 

- Monomial: `Monomial<P>` is a monomial (ex.: 5x1x4) with given coefficient and exponents (ex.: coefficient: 5, exponents: [1,0,0,1] ).
- Polynomial: `Polynomial<P>` is a set of monomial and represents polynomial over the field Fp.
- Multilinear Polynomial: It is a Polynomial with every varibale degrees at most 1.

`/protocol` - Contains specific implementations of the entities needed for this protocol

- Prover:`Prover<P>` implements the prover side of the Sum-Check protocol. Constructing gj through fixing the randomized rn got from the verifier.
- Verifier: `Verifier<P>` implements the verifier side of the Sum-Check protocol with each round verification and the generation of the rn used after round 1.
- Protocol: `SumCheck<P>` implements the simulation of interaction between Prover and Verifier as described above.
- Oracle: `Oracle<P>` abstraction that should be available for verifier that implements the evaluation of a multilinear polynomial at point (r1,...,rn) 

`/examples`

- ExampleOrchestrator:`ExampleOrchestrator<P>` responsible for parsing inputs and run the requested examples described below.
- ToyExample: `ToyExample<P>` Initializes `Prover<P>`, `Verifie<P>`, `Oracle<P>` and `SumCheck<P>` and processes one example with an honest prover.
- MultilinearExtensionExample: `MultilinearExtensionExample<P>` implements one dummy generation of f_tilde given a polynomial.
- MultilinearExtensionFromEvaluations<P>: `MultilinearExtensionFromEvaluations<P>` responsible for generating the multilinear extension of a function f given its evaluations and the vector r as described in Exercise 3.4.

## Vision

The purpose of this application is to grasp knowledge about the considerations during the implementation of the Sum-Check Protocol while learning both Rust and Sum-Check Protocol (Interactive Proofs, Zero-Knowledge proofs,...). 

I chose Rust for this project both to deepen my understanding of the language and to gain practical experience implementing mathematical and cryptographic primitives in a systems-oriented programming language.

Note: Currently, the project contains a toy implementation of the Sumcheck protocol. The final verification round and several production-oriented optimizations are still under development.

## Limitations / Future Work

Since the main purpose is to learn, I am currently using u64 as integer type on the given prime (P) in FieldElements, all polynomials implementation and Prover and Verifier, considering the ease of using u128 cast to not overflow under Algebraic operations implemented at FieldElement.

Future work includes:

- Completing and refining the final verification round
- Improving prover/verifier abstractions
- Supporting larger and more general polynomials
- Exploring more efficient polynomial representations
- Exploring multilinear-extension representations
- Improving error handling and protocol validation

## Installation

Make sure you have Rust Compiler and Cargo installed. Once installed, you may run the following command to setup the project:

```shell
cargo test && cargo run
```
or only:

```shell
cargo run
```
On a succesfull run you should select the option to be tested:

```shell
 1 - ToyExample
 2 - Multilinear extension through evaluations and vector r (Exercise 3.4)
 3 - Dummy f tilde generation
 0 - Exit
```

## Environment varibales

RUST_LOG

defaults to info.

Available log levels include:
- error (Level 1)
- warn (Level 2)
- info (Level 3)
- debug (Level 4)
- trace (Level 5)

## References
This project was created as a learning exercise based on concepts from:
- *Proofs, Arguments and Zero-Knowledge* by Justin Thaler (https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf)
  - Specifically, the Sum-Check Protocol described in Chapter 4 starting with simpler examples.

