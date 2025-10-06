#![cfg_attr(any(feature = "documentation", docsrs), doctored::doctored)]

//! Categorize items.

#[doc(tag(
    text = "Doctored",
    href = "https://github.com/michaelni678/doctored",
    background(color = "#4470AD")
))]
/// This struct is tagged.
///
/// See [Untagged] for its counterpart.
pub struct Tagged;

/// This struct isn't tagged.
///
/// See [Tagged] for its counterpart.
pub struct Untagged;
