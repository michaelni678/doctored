#![cfg_attr(any(feature = "documentation", docsrs), doctored::doctored)]

//! Apply Rust syntax highlighting to a code block.
//!
//! The code block below cannot be doctested, since it has an external file
//! dependency. To get around this, the [ignore attribute](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes)
//! can be used. However, a tooltip appears on the generated documentation,
//! indicating the code block hasn't been tested.
//!
//! ```ignore
//! let bytes = include_bytes!("spanish.in");
//! assert_eq!(bytes, b"adi\xc3\xb3s\n");
//! ```
//!
//! Like the `ignore` attribute, Doctored's `highlight` attribute also
//! syntax-highlights the code block and doesn't compile it. However, the
//! tooltip indicating the code block hasn't been tested is not shown.
//!
//! ```highlight
//! let bytes = include_bytes!("spanish.in");
//! assert_eq!(bytes, b"adi\xc3\xb3s\n");
//! ```
//!
//! When doctesting, code blocks with the `ignore` attribute are skipped.
//! Unfortunately, code blocks with the `highlight` attribute are not skipped.
//! However, the doctest is empty and will always pass.
//!
//! This can be useful for illustrating general structure. The code block below
//! demonstrates a hypothetical `fruits` procedural macro. The ellipsis (`...`)
//! in the attribute indicates optional arguments, and the ellipsis in the
//! struct fields indicates that more fields may be present.
//!
//! ```highlight
//! #[fruits(...)]
//! pub struct Fruits {
//!     apples: usize,
//!     oranges: usize,
//!     ...
//! }
//! ```
