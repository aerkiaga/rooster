# RCoC
An automated proof checker based on the Calculus of Constructions.

Written in Rust, proofs also employ a Rust-like syntax.
At the moment, it doesn't have many features, but it does work.

```rust
/* A proof that, for any three statements A, B and C, *
 * if A implies B and B implies C, then A implies C */

let proof_implication_is_transitive:
    @(A, B, C: Prop) {A -> B} -> {B -> C} -> {A -> C}
=
    |A, B, C: Prop, ab: A -> B, bc: B -> C| |x: A| bc(ab(x))
;
```

You can run the example file using the following command:

```
cargo run -- test.rcoc
```

Which will output:

```
proof_a = λT:Prop.λx:T.x
    :∀T:Prop.T→T
proof_b = λT:Prop.λP:Prop.λx:T.λy:T→P.y x
    :∀T:Prop.∀P:Prop.T→(T→P)→P
```

RCoC uses the amazing [chumsky](https://github.com/zesterer/chumsky)
and [ariadne](https://github.com/zesterer/ariadne) crates by @zesterer
to parse the input and produce meaningful error diagnostics.

The Calculus of Constructions was devised by Thierry Coquand,
and serves as the basis for [Coq](https://github.com/coq/coq)
and other proof assistants.
