# Sum-Check Protocol

## Protocol Description 

Suppose we are given a v-variate polynomial g defined over a finite field F. The purpose of the sum-check
protocol is for prover to provide the verifier with the following sum:

H := ∑b1∈{0,1} ∑b2∈{0,1} ... ∑bv∈{0,1} (g(b1,...,bv)).

• At the start of the protocol, the prover sends a value C1 claimed to equal the value H defined in Equation (4.1).
• In the first round, P sends the univariate polynomial g1(X1) claimed to equal ∑(x2,...,xv)∈{0,1}v−1 g(X1, x2,..., xv). V checks that C1 = g1(0) +g1(1), and that g1 is a univariate polynomial of degree at most deg1(g), rejecting if not. Here, degj(g) denotes the degree of g(X1,...,Xv) in variable Xj
• V chooses a random element r1 ∈ F, and sends r1 to P.
• In the jth round, for 1 < j < v, P sends to V a univariate polynomial gj(Xj) claimed to equal ∑(x j+1,...,xv)∈{0,1}v−j g(r1,...,rj−1,Xj, xj+1,..., xv). V checks that gj is a univariate polynomial of degree at most degj(g), and that gj−1(rj−1) = gj(0) +gj(1), rejecting if not.
• V chooses a random element rj ∈ F, and sends rj to P.
• In Round v, P sends to V a univariate polynomial gv(Xv) claimed to equal g(r1,...,rv−1,Xv). V checks that gv is a univariate polynomial of degree at most degv(g), rejecting if not, and also checks that gv−1(rv−1) = gv(0) +gv(1).
• V chooses a random element rv ∈ F and evaluates g(r1,...,rv) with a single oracle query to g.
V checks that gv(rv) = g(r1,...,rv), rejecting if not.
• If V has not yet rejected, V halts and accepts.

## Mathematic concepts vs implementation

`/field`

- Finite Field: Fp where p is prime and denote the set of integers modulo p. Used through `FieldElement<P>` for a chosen prime P under /field folder. Being p prime we are sure that adition, subtraction, multiplication and division work. Due to the fact that except for 0 all elements of the field are unitys (1, 2,...,p-1). It is later usefull for the use of polynomials.

`/polynomials`

- Monomial: `Monomial<P>` is a monomial (ex.: 5x1x4) with given coefficient and exponents (ex.: coefficient: 5, exponents: [] ).
- Polynomial: `Polynomial<P>` is a set of monomial and represents polynomial over the field Fp.
- Multilinear Polynomial: It is a Polynomial with every varibale degrees at most 1.

`/protocol`

- Prover:`Prover<P>` implements the prover side of the Sum-Check protocol. Currently reducing the multilinear polynomial to a univariate polynomial on each round.
- Verifier: `Verifier<P>` implements the verifier side of the Sum-Check protocol with each round verification and the generation of the rn used after round 1.

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

