#![cfg_attr(feature = "guide", doctored::doctored)]

#![rustfmt::skip]

//! Hides the summary in the module overview.
//!
//! # Example
#![doc(highlight)]
//! ```
#![doc(extras(include(documentation = "src/guide/attributes/summary/hide.Summarized.example")))]
//! pub struct Summarized;
//!
#![doc(extras(include(documentation = "src/guide/attributes/summary/hide.NotSummarized.example")))]
//! pub struct NotSummarized;
//! ```
//!
//! # Expansion
//!
//! See [`Summarized`] and [`NotSummarized`].

#[doc(extras(include(attributes = "src/guide/attributes/summary/hide.Summarized.example")))]
pub struct Summarized;

#[doc(extras(include(attributes = "src/guide/attributes/summary/hide.NotSummarized.example")))]
pub struct NotSummarized;
