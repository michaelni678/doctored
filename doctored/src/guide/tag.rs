#![cfg_attr(any(feature = "documentation", docsrs), doctored::doctored)]

//! Categorize items.

#[doc(tag(
    text = "Doctored",
    href = "https://github.com/michaelni678/doctored",
    color = "#4470AD",
))]
/// This struct is tagged.
///
/// See [Untagged] for its counterpart.
pub struct Tagged;

/// This struct is not tagged.
///
/// See [Tagged] for its counterpart.
pub struct Untagged;

pub mod tagged {
    #![doc(tag(
        text = "Doctored",
        href = "https://github.com/michaelni678/doctored",
        color = "#a144adff",
    ))]
    //! This module is tagged.
    //!
    //! See [untagged](super::untagged) for its counterpart.
}

pub mod untagged {
    //! This module is not tagged.
    //!
    //! See [tagged](super::tagged) for its counterpart.
}
