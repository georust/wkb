#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/georust/meta/master/logo/logo.png",
    html_favicon_url = "https://github.com/georust.png?size=32"
)]

mod common;
pub mod error;
pub mod reader;
#[cfg(test)]
mod test;
pub mod writer;

pub use common::Endianness;
