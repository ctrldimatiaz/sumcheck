# Sum-Check Protocol

A Rust implementation of the **Sum-Check Protocol**, developed as a learning project to explore interactive proofs, finite fields, polynomial arithmetic, and cryptographic primitives.

## Protocol Description

The central mathematical problem is computing the sum of a multivariate polynomial over the Boolean hypercube.

Given a polynomial \(g\) with \(v\) variables, the goal is to compute:

$$
H := \sum_{b_1 \in \{0,1\}}
\sum_{b_2 \in \{0,1\}}
\cdots
\sum_{b_v \in \{0,1\}}
g(b_1,\ldots,b_v)
$$

The prover initially claims a value $$C_1$$ equal to `H`.

In each subsequent round, the prover sends a univariate polynomial representing the appropriate partial sum of the original polynomial.

The verifier does not directly check whether each polynomial sent by the prover was correctly derived from the original polynomial \(g\). Instead, it checks consistency between consecutive rounds using randomly selected challenges.

These random challenges make it difficult for a dishonest prover to maintain consistency with a false initial claim.

### Protocol Interaction

The interaction between the prover and verifier can be represented as follows:

```text
       PROVER                              VERIFIER

       C₁ = claimed H
          ------------------------------->

          <------------------------------- Asks to prove it


       g₁(X₁)
          -------------------------------> Check:
                                          C₁ = g₁(0) + g₁(1)

          <------------------------------- Choose random r₁


       g₂(X₂)
          -------------------------------> Check:
                                          g₁(r₁) = g₂(0) + g₂(1)

          <------------------------------- Choose random r₂

                 ...

       gᵥ(Xᵥ)
          -------------------------------> Check:
                                          gᵥ₋₁(rᵥ₋₁) =
                                          gᵥ(0) + gᵥ(1)

          <------------------------------- Choose random rᵥ


       Final verification:

       The verifier checks:

       gᵥ(rᵥ) = g(r₁, ..., rᵥ)
```

### Construction of the Polynomials

At round `j`, the prover constructs a univariate polynomial $$g_j$$ by fixing the variables from previous rounds to the verifier's random challenges and summing over the remaining Boolean variables.

Formally:

$$
g_j(X_j) =
\sum_{x_{j+1},\ldots,x_v \in \{0,1\}}
g(r_1,\ldots,r_{j-1},X_j,x_{j+1},\ldots,x_v)
$$

For the first round, there are no previously fixed variables. For the final round, there are no remaining variables to sum over.

The verifier checks the following consistency conditions:

**First round:**

$$
C_1 = g_1(0) + g_1(1)
$$

**Intermediate rounds:**

$$
g_{j-1}(r_{j-1}) = g_j(0) + g_j(1)
$$

**Final round:**

$$
g_v(r_v) = g(r_1,\ldots,r_v)
$$

The final check is performed using an oracle that evaluates the original polynomial at the randomly selected point.

---

## Mathematical Concepts vs. Implementation

The project separates the mathematical concepts from their Rust implementations into different modules.

### `/field` — Finite Field Arithmetic

Representation of finite field elements and their algebraic operations.

- **`Finite Field`:** $$F_p$$, where `p` is prime, represents the integers modulo `p`.

- **`FieldElement<P>`:** Represents an element of the finite field for a chosen prime modulus `P`.

Because `p` is prime, the integers modulo `p` form a field. Addition, subtraction, and multiplication are closed operations, and every nonzero element has a multiplicative inverse, allowing division by nonzero elements.

### `/polynomials` — Polynomial Representations

Representation of the polynomials required by the protocol.

- **`Monomial<P>`:** Represents a monomial with a coefficient and a vector of exponents.

  For example:

  5 $$x_1$$ $$x_4$$

  Can be represented by:

  ```rust
  coefficient: 5
  exponents: [1, 0, 0, 1]
  ```

- **`Polynomial<P>`:** Represents a polynomial over the finite field \(\mathbb{F}_p\) as a collection of monomials.

- **`MultilinearPolynomial<P>`:** Represents a multilinear polynomial, meaning that the degree of every individual variable is at most one.

### `/protocol` — Sum-Check Protocol Implementation

Contains the main entities responsible for implementing the protocol.

- **`Prover<P>`:** Implements the prover's side of the Sum-Check protocol. Constructs the partial sum polynomials \(g_j\) by fixing previously randomized variables using the challenges received from the verifier.

- **`Verifier<P>`:** Implements the verifier's side of the protocol, including the consistency checks performed in each round and the generation of random challenges.

- **`SumCheck<P>`:** Coordinates the interaction between the prover and verifier, simulating the protocol described above.

- **`Oracle<P>`:** Provides an abstraction for the verifier to evaluate the original multilinear polynomial at a given point \((r_1,\ldots,r_v)\).

### `/examples` — Usage Examples

Contains examples demonstrating the implemented mathematical concepts and protocol.

- **`ExampleOrchestrator<P>`:** Responsible for parsing user input and running the requested examples.

- **`ToyExample<P>`:** Initializes `Prover<P>`, `Verifier<P>`, `Oracle<P>`, and `SumCheck<P>`, and executes an example using an honest prover.

- **`MultilinearExtensionExample<P>`:** Implements a dummy generation of \(\widetilde{f}\) given a polynomial.

- **`MultilinearExtensionFromEvaluations<P>`:** Generates the multilinear extension of a function \(f\) from its evaluations and a vector \(r\), as described in Exercise 3.4.

- **`DishonestProver<P>`:** Initializes `Prover<P>`, `Verifier<P>`, `Oracle<P>`, and `SumCheck<P>`, and executes an example using an dishonest prover where Prover and Oracle multilinear polynomial are different.
---

## Vision

The purpose of this application is to gain a deeper understanding of the considerations involved in implementing the Sum-Check Protocol while learning both Rust and the underlying mathematical concepts.

I chose Rust for this project to deepen my understanding of the language and gain practical experience implementing mathematical and cryptographic primitives in a systems-oriented programming language.

**Current implementation status:** The project currently contains a Toy Implementation of the Sum-Check protocol, a Dishonest Prover implementation, a Multilinear extension evaluation of f tilde according to Exercise 3.2 in Justin Thaler's book referenced below and a dummy multilinear polynomial extension implementation. Several production-oriented optimizations are still under development.

---

## Limitations and Future Work

Since the primary purpose of this project is learning, I am currently using `u64` as the integer representation for field elements, polynomial coefficients, and the prover/verifier operations.

For arithmetic operations, I use `u128` intermediate values to avoid overflow when multiplying two `u64` field elements before reducing the result modulo the prime.

This approach simplifies the implementation but limits the range of supported prime moduli and is not intended as a general-purpose finite field arithmetic implementation.

### Future Improvements

- [ ] Improving prover and verifier abstractions.
- [ ] Supporting larger and more general polynomials.
- [ ] Exploring more efficient polynomial representations.
- [ ] Exploring alternative multilinear extension representations.
- [ ] Improving error handling and protocol validation.
- [ ] Improving arithmetic support for larger finite fields.
- [ ] Adding more comprehensive tests and examples.

---

## Installation

Make sure you have the Rust compiler and Cargo installed.

Clone the repository and navigate to the project directory.

To run the tests and execute the application:

```bash
cargo test && cargo run
```

Alternatively, to execute the application without running the tests:

```bash
cargo run
```

After a successful execution, select the example you want to run:

```text
1 - ToyExample
2 - Multilinear extension through evaluations and vector r (Exercise 3.3/3.4)

    "Example of Exercise 3.3 inputs:"
    Insert f evaluations array (ex.: 1,2,4,8 (r²)):
    1,2,3,4,5,6,7,8
    Insert vector r to be evaluated (ex.: 1,2):
    2,4,6

3 - Dummy f tilde generation
4 - Dishonest Prover
0 - Exit
```

---

## Environment Variables

### `RUST_LOG`

Controls the application's logging verbosity.

Defaults to `info`.

Available log levels:

| Level | Description |
|---|---|
| `error` | Error messages |
| `warn` | Warnings |
| `info` | General informational messages |
| `debug` | Debugging information |
| `trace` | Detailed execution traces |

For example, to run the application with debug logging:

```bash
RUST_LOG=debug cargo run
```

---

## References

This project was created as a learning exercise based on concepts from:

- **[Proofs, Arguments and Zero-Knowledge](https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf)** by Justin Thaler.
  - Specifically, the Sum-Check Protocol described in Chapter 4, starting with the simpler examples.
