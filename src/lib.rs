//! # Decimal Floating Point Arithmetic for Rust
//! based on bindings for **Intel(R) Decimal Floating-Point Math Library v2.2**

extern crate dfp_number_sys;

mod decimal128;

pub use decimal128::*;
