#![cfg_attr(feature = "guide", doctored::doctored)]

#![rustfmt::skip]

//! Shows a fake summary.
//!
//! # Example
#![doc(highlight)]
//! ```
#![doc(extras(include(documentation = "src/guide/attributes/summary/mock.Mocked.example")))]
//! pub struct Mocked;
//!
#![doc(extras(include(documentation = "src/guide/attributes/summary/mock.NotMocked.example")))]
//! pub struct NotMocked;
//! ```
//!
//! # Expansion
//!
//! See [`Mocked`] and [`NotMocked`].

#[doc(extras(include(attributes = "src/guide/attributes/summary/mock.Mocked.example")))]
pub struct Mocked;

#[doc(extras(include(attributes = "src/guide/attributes/summary/mock.NotMocked.example")))]
pub struct NotMocked;
