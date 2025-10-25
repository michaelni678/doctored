#![cfg_attr(feature = "guide", doctored::doctored)]
#![doc(highlight)]
//! Syntax highlighting.
//!
//! All code blocks marked with a supported language will be highlighted.
//!
//! Doctored uses [highlight.js](https://github.com/highlightjs/highlight.js)
//! under the hood.
//!
//! # Theme
//!
//! Syntax highlighting uses a theme imitating after Rustdoc. Many
//! [HLJS scopes](https://highlightjs.readthedocs.io/en/latest/css-classes-reference.html)
//! are still unstyled, so if anything looks off, please
//! [open an issue](https://github.com/michaelni678/doctored/issues?q=sort%3Aupdated-desc+is%3Aissue+is%3Aopen).
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
