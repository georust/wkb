use std::io::Cursor;

use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::util::{bounded_capacity, has_srid, ReadBytesExt, MIN_WKB_GEOMETRY_SIZE};
use crate::reader::{Wkb, HEADER_BYTES, MAX_WKB_NESTING_DEPTH};
use crate::Endianness;
use geo_traits::GeometryCollectionTrait;

/// A WKB GeometryCollection
#[derive(Debug, Clone)]
pub struct GeometryCollection<'a> {
    /// A WKB object for each of the internal geometries
    geometries: Vec<Wkb<'a>>,
    buf: &'a [u8],
    dim: Dimension,
}

impl<'a> GeometryCollection<'a> {
    /// Construct a new GeometryCollection from a WKB buffer.
    ///
    /// This will parse the WKB header and extract all contained geometries.
    pub(crate) fn try_new(
        buf: &'a [u8],
        byte_order: Endianness,
        dim: Dimension,
        depth: usize,
    ) -> WkbResult<Self> {
        if depth >= MAX_WKB_NESTING_DEPTH {
            return Err(WkbError::NestingDepthExceeded(MAX_WKB_NESTING_DEPTH));
        }

        let has_srid = has_srid(buf, byte_order)?;
        let num_geometries_offset = HEADER_BYTES + if has_srid { 4 } else { 0 };

        let mut reader = Cursor::new(buf);
        reader.set_position(num_geometries_offset);
        let num_geometries = reader
            .read_u32(byte_order)?
            .try_into()
            .map_err(|e| WkbError::General(format!("Invalid number of geometries: {}", e)))?;

        let mut geometry_offset = num_geometries_offset as usize + 4;

        let capacity = bounded_capacity(
            num_geometries,
            buf.len().saturating_sub(geometry_offset),
            MIN_WKB_GEOMETRY_SIZE,
        );
        let mut geometries = Vec::with_capacity(capacity);
        for _ in 0..num_geometries {
            let geometry = Wkb::try_new_with_depth(&buf[geometry_offset..], depth + 1)?;
            geometry_offset += geometry.size() as usize;
            geometries.push(geometry);
        }

        Ok(Self {
            geometries,
            buf: &buf[0..geometry_offset],
            dim,
        })
    }

    /// The dimension of this GeometryCollection
    pub fn dimension(&self) -> Dimension {
        self.dim
    }

    /// The number of bytes in this object, including any header
    #[inline]
    pub fn size(&self) -> u64 {
        self.buf.len() as u64
    }

    /// Return the underlying buffer of this GeometryCollection.
    #[inline]
    pub fn buf(&self) -> &'a [u8] {
        self.buf
    }
}

impl<'a> GeometryCollectionTrait for GeometryCollection<'a> {
    type GeometryType<'b>
        = &'b Wkb<'a>
    where
        Self: 'b;

    fn num_geometries(&self) -> usize {
        self.geometries.len()
    }

    unsafe fn geometry_unchecked(&self, i: usize) -> Self::GeometryType<'_> {
        self.geometries.get_unchecked(i)
    }
}
