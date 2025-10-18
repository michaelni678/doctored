#![cfg_attr(feature = "guide", doctored::doctored)]
//! Doctored user guide.
//!
//! # Caution
//!
//! Doctored relies on internal Rustdoc behavior, which may change in future
//! releases. Updates to Rustdoc could potentially break Doctored's
//! functionality.
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
//! ## Outer Documentation
//!
//! To use Doctored on [outer documentation](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html#outer-and-inner-documentation),
//! simply put the `doctored` attribute macro at the top of the documentation
//! attributes.
#![doc(highlight)]
//! ```
//! #[doctored::doctored]
//! #[doc(tag(text = "Module"))]
//! /// Hello, World!
//! pub mod example {}
//! ```
//!
//! ## Inner Documentation
//!
//! Unfortunately, using Doctored on [inner documentation](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html#outer-and-inner-documentation)
//! is more limiting and prone to bugs since the [`custom_inner_attributes`](https://doc.rust-lang.org/beta/unstable-book/language-features/custom-inner-attributes.html)
//! feature is unstable.
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
#![doc(highlight)]
//! ```
//! #![cfg_attr(
//!     feature = "nightly",
//!     feature(custom_inner_attributes, proc_macro_hygiene)
//! )]
//! ```
//!
//! Add the snippet below to the top of modules that use Doctored.
#![doc(highlight)]
//! ```
//! #![cfg_attr(feature = "nightly", doctored::doctored)]
//! ```
//!
//! # Attributes
//!
//! Doctored provides several `#[doc(...)]` attributes to enhance your
//! documentation.
//!
//! See the [attributes] module for more information!

pub mod attributes;
