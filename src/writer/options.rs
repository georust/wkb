use crate::Endianness;

/// Options for writing geometries to WKB
#[derive(Debug, Clone, Default)]
pub struct WriteOptions {
    /// The byte order to use when writing the WKB
    pub endianness: Endianness,
}
