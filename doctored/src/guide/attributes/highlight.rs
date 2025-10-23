#![cfg_attr(feature = "guide", doctored::doctored)]

//! Syntax highlighting for code blocks.
//!
//! The `highlight` attribute argument applies Rust syntax highlighting to code
//! blocks. When the code block is doctested, the test will always pass,
//! regardless of whether the code can compile.
//!
//! This behaves similarly to Rustdoc's built-in [`ignore` attribute](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes).
//! The key difference is that doctests marked with `ignore` display a tooltip
//! stating that the code block was not tested, whereas those generated with
//! Doctored's `highlight` do not.
//!
//! Doctored scans documentation for code blocks marked with the `highlight`
//! attribute.
//!
#![doc(highlight)]
//! ```
//! //! ```highlight
//! //! pub fn foo() {}
//! //! ```
//! ```
//!
//! You can also use an actual attribute placed directly above a code block.
//!
#![doc(highlight)]
//! ```
//! #![doc(highlight)]
//! //! ```
//! //! pub fn foo() {}
//! //! ```
//! ```
//!
//! # Example
//!
#![doc(highlight)]
//! ```
#![doc(extras(include(documentation = "src/guide/attributes/highlight.example")))]
//! ```
//!
//! # Expansion
//!
#![doc(extras(include(attributes = "src/guide/attributes/highlight.example")))]
