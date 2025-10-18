#![cfg_attr(
    feature = "guide",
    feature(doc_cfg, custom_inner_attributes, proc_macro_hygiene)
)]

//! Rust documentation utilities.
//!
//! See the [guide] for help!

extern crate self as doctored;

pub use doctored_macros::doctored;

#[cfg(feature = "guide")]
pub mod guide;
