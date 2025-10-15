#![cfg_attr(any(feature = "documentation", docsrs), doctored::doctored)]

//! Clipboard operations.
//!
//! # Copy
//!
//! The `copy` operation copies all documentation between a matching head and
//! tail under the specified name.
//!
//! # Paste
//!
//! The `paste` operation pastes content under the specified name.
//!
//! # Modifiers
//!
//! When copying and pasting documentation, modifiers can be applied to change
//! the documentation.
//!
//! When a modifier is applied to a copy operation, all
//! pastes will have the modification. When a modifier is applied to a paste
//! operation, only the single paste will have the modification. Multiple
//! modifiers can be supplied, and they are applied in the order of declaration.
//!
//! | Modifier                  | Description                                       |
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
#![doc(extras(include(documentation = "doctored/src/guide/clipboard.copy.example")))]
//! ```
//! 
//! # Expansion
#![doc(extras(include(attributes = "doctored/src/guide/clipboard.copy.example")))]
