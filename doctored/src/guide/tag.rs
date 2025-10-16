#![cfg_attr(feature = "guide", doctored::doctored)]

#![rustfmt::skip]

//! Categorize items.
//!
//! # Example
#![doc(highlight)]
//! ```
#![doc(extras(include(documentation = "doctored/src/guide/tag.Tagged.example")))]
//! pub struct Tagged;
//!
#![doc(extras(include(documentation = "doctored/src/guide/tag.HyperlinkTagged.example")))]
//! pub struct HyperlinkTagged;
//!
#![doc(extras(include(documentation = "doctored/src/guide/tag.NotTagged.example")))]
//! pub struct NotTagged;
//! ```
//!
//! # Expansion
//!
//! See [`Tagged`], [`HyperlinkTagged`], and [`NotTagged`].


#[doc(extras(include(attributes = "doctored/src/guide/tag.Tagged.example")))]
pub struct Tagged;

#[doc(extras(include(attributes = "doctored/src/guide/tag.HyperlinkTagged.example")))]
pub struct HyperlinkTagged;

#[doc(extras(include(attributes = "doctored/src/guide/tag.NotTagged.example")))]
pub struct NotTagged;
