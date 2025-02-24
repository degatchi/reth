//! Module that interacts with MDBX.

#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

/// Rust bindings for [MDBX](https://libmdbx.dqdkfa.ru/).
pub mod mdbx {
    pub use reth_libmdbx::*;
}

pub mod kv;
mod utils;
