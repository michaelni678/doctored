#![cfg_attr(feature = "guide", doctored::doctored)]

//! Categorize items.
//!
//! # Arguments
//!
//! | Arguments         | Description                  |
//! |-------------------|------------------------------|
//! | `color = "color"` | Sets the color of the tag.   |
//! | `href = "href"`   | Adds a hyperlink to the tag. |
//!
//! ## Color
//!
//! Any [CSS color](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value)
//! can be used. If no color is specified, the tag will default to `steelblue`.
//!
//! # Example
//! 
#![doc(highlight)]
//! ```
#![doc(extras(include(documentation = "src/guide/attributes/tag.Tagged.example")))]
//! pub struct Tagged;
//!
#![doc(extras(include(documentation = "src/guide/attributes/tag.HyperlinkTagged.example")))]
//! pub struct HyperlinkTagged;
//!
#![doc(extras(include(documentation = "src/guide/attributes/tag.NotTagged.example")))]
//! pub struct NotTagged;
//! ```
//!
//! # Expansion
//!
//! See [`Tagged`], [`HyperlinkTagged`], and [`NotTagged`].

#[doc(extras(include(attributes = "src/guide/attributes/tag.Tagged.example")))]
pub struct Tagged;

#[doc(extras(include(attributes = "src/guide/attributes/tag.HyperlinkTagged.example")))]
pub struct HyperlinkTagged;

#[doc(extras(include(attributes = "src/guide/attributes/tag.NotTagged.example")))]
pub struct NotTagged;
