#![cfg_attr(feature = "guide", doctored::doctored)]
#![doc(highlight)]
//! Syntax highlighting.
//!
//! All code blocks marked with a supported language will be highlighted.
//!
//! Doctored uses [highlight.js](https://github.com/highlightjs/highlight.js)
//! under the hood.
//!
//! # Example
//!
#![doc(disregard)]
//! ```
//! #![doc(highlight)]
#![doc(extras(include(documentation = "src/guide/attributes/highlight.example")))]
//! ```
//!
//! # Expansion
//!
#![doc(extras(include(attributes = "src/guide/attributes/highlight.example")))]
