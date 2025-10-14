#![cfg_attr(any(feature = "documentation", docsrs), doctored::doctored)]

//! Copies documentation.
//!
//! Used in tandem with the [paste](super::paste) attribute argument.
//!
//! # Example
//!
//! ## Source Code
#![doc(highlight)]
//! ```
#![doc(extras(include(documentation = "doctored/src/guide/clipboard/copy.example")))]
//! ```
//! 
//! ## Generated Docs
#![doc(extras(include(attributes = "doctored/src/guide/clipboard/copy.example")))]
