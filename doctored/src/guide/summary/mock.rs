#![cfg_attr(docsrs, doctored::doctored)]

//! Shows a fake summary.

pub mod unmocked {
    //! This summary appears in the documentation!
    //!
    //! The text above appears in the module overview, as well as on the actual
    //! documentation.
    //!
    //! See [mocked](super::mocked) for its counterpart.
}

pub mod mocked {
    #![doc(summary(mock = "This summary doesn't appear in the documentation!"))]
    //! This is the first line, but the summary in the module overview is
    //! different!
    //!
    //! See [unmocked](super::unmocked) for its counterpart.
}

/// This summary appears in the documentation!
///
/// The text above appears in the module overview, as well as on the actual
/// documentation.
///
/// See [Mocked](super::Mocked) for its counterpart.
pub struct Unmocked;

#[doc(summary(mock = "This summary doesn't appear in the documentation!"))]
/// This is the first line, but the summary in the module overview is
/// different!
///
/// See [Unmocked](super::Unmocked) for its counterpart.
pub struct Mocked;
