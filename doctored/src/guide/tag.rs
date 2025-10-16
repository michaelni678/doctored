#![cfg_attr(feature = "guide", doctored::doctored)]

//! Categorize items.

#[doc(tag(text = "Struct", color = "#44ad67ff"))]
/// This struct is tagged.
///
/// See [Untagged] for its counterpart.
pub struct Tagged;

/// This struct is not tagged.
///
/// See [Tagged] for its counterpart.
pub struct Untagged;

pub mod tagged {
    #![doc(tag(text = "Module", href = "https://www.youtube.com/watch?v=dQw4w9WgXcQ"))]
    //! This module is tagged with a hyperlink.
    //!
    //! See [untagged](super::untagged) for its counterpart.
}

pub mod untagged {
    //! This module is not tagged.
    //!
    //! See [tagged](super::tagged) for its counterpart.
}
