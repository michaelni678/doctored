#![cfg_attr(feature = "guide", doctored::doctored)]

//! Clipboard operations.
//!
//! Copying and pasting documentation helps maintain a single source of truth.
//! This is especially useful when multiple doctests contain repeated
//! boilerplate code.
//!
//! # Copy Operation
//!
//! The `copy` operation copies all documentation between a matching head and
//! tail under the specified name.
//!
//! ## Arguments
//!
//! | Arguments               | Description                                                 |
//! |-------------------------|-------------------------------------------------------------|
//! | `head = "name"`         | Marks the start of content copied under the specified name. |
//! | `tail = "name"`         | Marks the end of content copied under the specified name.   |
//! | [Modifiers](#modifiers) | Modifies each line of documentation prior to copying.       |
//!
//! # Paste Operation
//!
//! The `paste` operation pastes content under the specified name.
//!
//! ## Arguments
//!
//! | Arguments               | Description                                           |
//! |-------------------------|-------------------------------------------------------|
//! | `name = "name"`         | Pastes content copied under the specified name.       |
//! | [Modifiers](#modifiers) | Modifies each line of documentation prior to pasting. |
//!
//! # Modifiers
//!
//! When copying and pasting documentation, modifiers can be applied to change
//! the documentation. If multiple modifiers are supplied, they are applied in
//! the order of declaration.
//!
//! ## Arguments
//!
//! | Arguments                 | Description                                       |
//! |---------------------------|---------------------------------------------------|
//! | `strip`                   | Strips whitespace on both sides of each line.     |
//! | `strip(left)`             | Strips whitespace on the left side of each line.  |
//! | `strip(left = "prefix")`  | Strips a prefix on the left side of each line.    |
//! | `strip(right)`            | Strips whitespace on the right side of each line. |
//! | `strip(right = "suffix")` | Strips a suffix on the right side of each line.   |
//! | `push(left = "prefix")`   | Pushes a prefix on the left side of each line.    |
//! | `push(right = "suffix")`  | Pushes a suffix on the right side of each line.   |
//!
//! # Example
#![doc(highlight)]
//! ```
#![doc(extras(include(documentation = "doctored/src/guide/attributes/clipboard.copy.example")))]
//! ```
//! 
//! # Expansion
#![doc(extras(include(attributes = "doctored/src/guide/attributes/clipboard.copy.example")))]
