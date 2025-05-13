#![doc = include_str!("../README.md")]

mod common;
pub mod error;
pub mod reader;
#[cfg(test)]
mod test;
pub mod writer;

pub use common::Endianness;
