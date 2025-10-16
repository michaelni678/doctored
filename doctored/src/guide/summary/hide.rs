#![cfg_attr(feature = "guide", doctored::doctored)]

//! Hides the summary in the module overview.
//! 
//! # Example
#![doc(highlight)]
//! ```
#![doc(extras(include(documentation = "doctored/src/guide/summary/hide.Summarized.example")))]
//! pub struct Summarized;
//! 
#![doc(extras(include(documentation = "doctored/src/guide/summary/hide.NotSummarized.example")))]
//! pub struct NotSummarized;
//! ```
//! 
//! # Expansion
//!
//! See [`Summarized`] and [`NotSummarized`].

#[doc(extras(include(attributes = "doctored/src/guide/summary/hide.Summarized.example")))]
pub struct Summarized;

#[doc(extras(include(attributes = "doctored/src/guide/summary/hide.NotSummarized.example")))]
pub struct NotSummarized;
