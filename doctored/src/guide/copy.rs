#![cfg_attr(docsrs, doctored::doctored)]

//! Copies documentation.
//!
//! The `copy` attribute argument is used to copy documentation between a `head`
//! and `tail`. Every `head` must have a `tail` with the same tag. The
//! [`paste`](super::paste) argument can be used to paste the copied
//! documentation.
//!
//! Multiple parts of the code block below are copied and pasted.
//!
//! ```
#![doc(copy(head("all", "struct")))]
//! pub struct Copied {
//!     x: i32,
//!     y: String,
//! }
#![doc(copy(tail("struct")))]
//!
#![doc(copy(head("impl")))]
//! impl Copied {
//!     pub fn foo() {}
//! }
#![doc(copy(tail("all", "impl")))]
//! ```
//! 
//! `#[doc(paste("struct"))]`:
//! ```
#![doc(paste("struct"))]
//! ```
//! 
//! `#[doc(paste("impl"))]`:
//! ```
#![doc(paste("impl"))]
//! ```
//! 
//! `#[doc(paste("all"))]`:
//! ```
#![doc(paste("all"))]
//! ```
