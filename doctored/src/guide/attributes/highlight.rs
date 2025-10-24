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
//! You can specify a HLJS theme to customize the syntax highlighting
//! style.
//!
//! The code block below uses the `grayscale.min` theme.
//!
#![doc(disregard)]
//! ```
//! // Theme: 
//! // https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/grayscale.min.css.
//!
//! #![doc(highlight = "grayscale.min")]
//! //! ```java
//! //! public class Main {
//! //!     public static void main(String[] args) {
//! //!         System.out.println("Hello, World!");
//! //!     }
//! //! }
//! //! ```
//! ```
//!
//! The theme defaults to [`Base16 Atelier Dune Light`](https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/base16/atelier-dune-light.min.css)
//! by Bram de Haan.
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
