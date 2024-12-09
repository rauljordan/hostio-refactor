# Stylus Host I/O Check

This PR has a simple proof-of-concept using once_cell's Lazy implementation and AtomicCell to provide interior
mutability for statics that need to be Send in the Stylus SDK.

Rust 1.83 added a `deny` rule to the compiler that blocks compilation of any attempt to grab a reference to
a mutable static. The only viable alternative are raw pointers or interior mutability.

See more [here](https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html) about the
changes to Rust 1.83 and the recommendations.

This repo proves we can build and run these hostios. However, we still need to cross-check against wasm.