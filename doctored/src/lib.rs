#![cfg_attr(
    feature = "guide",
    feature(doc_cfg, custom_inner_attributes, proc_macro_hygiene)
)]

//! Rust documentation utilities.
//!
//! See the [guide] for more information!
//!
//! # Quick Start
//!
//! Add Doctored to your Cargo.toml.
//!
//! ```toml
//! [dependencies]
//! doctored = "0.1.0"
//! ```
//!
//! Add a feature to your Cargo.toml. Unfortunately, you'll need to enable this
//! feature whenever Doctored's macros should be expanded.
//!
//! ```toml
//! [features]
//! nightly = []
//!
//! [package.metadata.docs.rs]
//! features = ["nightly"]
//! ```
//!
//! At the top of the crate root, enable the nightly `custom_inner_attributes`
//! and `proc_macro_hygiene` features.
//!
//! ```text
//! #![cfg_attr(
//!     feature = "nightly",
//!     feature(custom_inner_attributes, proc_macro_hygiene)
//! )]
//! ```
//!
//! Add the snippet below to the top of modules that use Doctored.
//!
//! ```text
//! #![cfg_attr(feature = "nightly", doctored::doctored)]
//! ```
//!
//! # Disclaimers
//!
//! 1. Doctored depends on multiple unstable features. Expect bugs.
//! 2. Doctored relies on internal Rustdoc behavior, which may change in future
//!    releases. Updates to Rustdoc could potentially break functionality.
//!
//! See the [guide] for more information!

extern crate self as doctored;

pub use doctored_macros::doctored;

#[cfg(feature = "guide")]
pub mod guide;
