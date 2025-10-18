#![cfg_attr(feature = "guide", doctored::doctored)]

//! Syntax highlighting for code blocks.
//!
//! The `highlight` attribute argument applies Rust syntax highlighting to code
//! blocks. When the code block is doctested, the test will always pass,
//! regardless of whether the code can compile.
//!
//! This behaves similarly to Rust's built-in [`ignore` attribute](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes).
//! The key difference is that doctests marked with `ignore` display a tooltip
//! stating that the code block was not tested, whereas those generated with
//! `highlight` do not.
//!
//! # Example
#![doc(highlight)]
//! ```
#![doc(extras(include(documentation = "doctored/src/guide/attributes/highlight.example")))]
//! ```
//! 
//! # Expansion
#![doc(extras(include(attributes = "doctored/src/guide/attributes/highlight.example")))]
