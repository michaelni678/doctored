#![cfg_attr(any(feature = "documentation", docsrs), doctored::doctored)]

//! Copies documentation.
//!
//! The code below defines a function `middle`, which finds the integer in the
//! center of the two given integers.
//!
//! ```
#![doc(clipboard(copy(head = "function")))]
//! pub fn middle(a: usize, b: usize) -> usize {
//!     assert!(a <= b);
#![doc(clipboard(copy(head = "equation", strip)))]
//!     a + (b - a) / 2
#![doc(clipboard(copy(tail = "equation")))]
//! }
#![doc(clipboard(copy(tail = "function")))]
//! ```
//! 
//! This is a doctest for the `middle` function.
//! ```
#![doc(clipboard(paste(tag = "function", lpush = "#")))]
//! assert_eq!(middle(6, 8), 7);
//! ```
//! 
//! To avoid integer overflow, the middle is calculated with the equation `
#![doc(clipboard(paste(tag = "equation")))]
//!`.
