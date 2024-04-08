#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[doc(hidden)]
pub mod testing_prelude;

mod fail_all;
pub use fail_all::{fail_all, fail_all_vec};

// Implicitly does `#[macro_export]` of `return_multiple_errors!`
mod return_multiple_errors;
