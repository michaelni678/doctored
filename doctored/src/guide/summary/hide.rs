#![cfg_attr(feature = "guide", doctored::doctored)]

//! Hides the summary in the module overview.

pub mod shown {
    //! This summary is shown!
    //!
    //! The text above appears in the module overview.
    //!
    //! See [hidden](super::hidden) for its counterpart.
}

pub mod hidden {
    #![doc(summary(hide))]
    //! This summary is hidden!
    //!
    //! Even though the text above is on the first line, it does not appear in
    //! the module overview.
    //!
    //! See [shown](super::shown) for its counterpart.
}

/// This summary is shown!
///
/// The text above appears in the module overview.
///
/// See [`Hidden`] for its counterpart.
pub struct Shown;

#[doc(summary(hide))]
/// This summary is hidden!
///
/// Even though the text above is on the first line, it does not appear in the
/// module overview.
///
/// See [`Shown`] for its counterpart.
pub struct Hidden;
