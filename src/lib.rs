//! # Decimal Floating Point Arithmetic for Rust

pub mod dec_common;
pub mod dec_context;
pub mod dec_quad;
mod decimal128;
pub mod dpd;
pub mod quad_conv;
pub mod tables;

pub use decimal128::Decimal128;
