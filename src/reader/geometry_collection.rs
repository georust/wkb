use std::io::Cursor;

use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::util::{has_srid, ReadBytesExt};
use crate::reader::{Wkb, HEADER_BYTES};
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
    ) -> WkbResult<Self> {
        let has_srid = has_srid(buf, byte_order)?;
        let num_geometries_offset = HEADER_BYTES + if has_srid { 4 } else { 0 };

        let mut reader = Cursor::new(buf);
        reader.set_position(num_geometries_offset);
        let num_geometries = reader
            .read_u32(byte_order)?
            .try_into()
            .map_err(|e| WkbError::General(format!("Invalid number of geometries: {}", e)))?;

        let mut geometry_offset = num_geometries_offset as usize + 4;

        let mut geometries = Vec::with_capacity(num_geometries);
        for _ in 0..num_geometries {
            let geometry = Wkb::try_new(&buf[geometry_offset..])?;
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
