#![cfg_attr(any(feature = "documentation", docsrs), doctored::doctored)]

//! Apply Rust syntax highlighting to a code block.
//!
//! The [ignore attribute](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes)
//! can be used to skip a doctest. However, a tooltip appears on the generated
//! documentation, indicating the code block hasn't been tested.
//!
//! ```ignore
//! let bytes = include_bytes!("spanish.in");
//! assert_eq!(bytes, b"adi\xc3\xb3s\n");
//! ```
//!
//! Like the `ignore` attribute, Doctored's `highlight` attribute also
//! syntax-highlights the code block and doesn't compile it. However, the
//! tooltip indicating the code block hasn't been tested is not shown.
#![doc(highlight)]
//! ```
//! let bytes = include_bytes!("spanish.in");
//! assert_eq!(bytes, b"adi\xc3\xb3s\n");
//! ```
//!
//! When doctesting, code blocks with the `ignore` attribute are skipped.
//! Unfortunately, code blocks with the `highlight` attribute are not skipped.
//! However, the doctest is empty and should always pass.
//!
//! # Syntax
//!
//! The `highlight` attribute can be used in the same way as the built-in
//! doctest attributes.
//!
//! ```
//! /// ```highlight
//! /// fn foo() {}
//! /// ```
//! ```
//!
//! Alternatively, you can use an actual attribute.
//!
//! ```
//! #[doc(highlight)]
//! /// ```
//! /// fn foo() {}
//! /// ```
//! ```
//!
//! # Use Cases
//!
//! This can be useful for illustrating general structure. The code block below
//! demonstrates a hypothetical `fruits` procedural macro. The ellipsis (`...`)
//! in the attribute indicates optional arguments, and the ellipsis in the
//! struct fields indicates that more fields may be present.
#![doc(highlight)]
//! ```
//! #[fruits(...)]
//! pub struct Fruits {
//!     apples: usize,
//!     oranges: usize,
//!     ...
//! }
//! ```
//!
//! # Example
//!
//! Below is a demonstration of the `highlight` attribute.
//!
//! ## Source Code
#![doc(highlight)]
//! ```
#![doc(extras(include(documentation = "doctored/src/guide/highlight.example")))]
//! ```
//! 
//! ## Generated Docs
#![doc(extras(include(attributes = "doctored/src/guide/highlight.example")))]
