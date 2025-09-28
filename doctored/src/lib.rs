#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(any(doc, docsrs), feature(custom_inner_attributes, proc_macro_hygiene))]

extern crate self as doctored;

pub use doctored_macros::doctored;

#[cfg(feature = "guide")]
pub mod guide;
