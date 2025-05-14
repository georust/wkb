use std::io::Cursor;

use crate::common::Dimension;
use crate::error::WkbResult;
use crate::reader::util::{has_srid, ReadBytesExt};
use crate::reader::Wkb;
use crate::Endianness;
use geo_traits::GeometryCollectionTrait;

/// skip endianness and wkb type
const HEADER_BYTES: u64 = 5;

/// A WKB GeometryCollection
#[derive(Debug, Clone)]
pub struct GeometryCollection<'a> {
    /// A WKB object for each of the internal geometries
    geometries: Vec<Wkb<'a>>,
    dim: Dimension,
    has_srid: bool,
}

impl<'a> GeometryCollection<'a> {
    pub fn try_new(buf: &'a [u8], byte_order: Endianness, dim: Dimension) -> WkbResult<Self> {
        let mut offset = 0;
        let has_srid = has_srid(buf, byte_order, offset);
        if has_srid {
            offset += 4;
        }

        let mut reader = Cursor::new(buf);
        reader.set_position(HEADER_BYTES + offset);
        let num_geometries = reader.read_u32(byte_order).unwrap().try_into().unwrap();

        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numGeometries
        let mut geometry_offset = 1 + 4 + 4;
        if has_srid {
            geometry_offset += 4;
        }

        let mut geometries = Vec::with_capacity(num_geometries);
        for _ in 0..num_geometries {
            let geometry = Wkb::try_new(&buf[geometry_offset..])?;
            geometry_offset += geometry.size() as usize;
            geometries.push(geometry);
        }

        Ok(Self {
            geometries,
            dim,
            has_srid,
        })
    }

    pub fn dimension(&self) -> Dimension {
        self.dim
    }

    pub fn size(&self) -> u64 {
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numGeometries
        let mut header = 1 + 4 + 4;
        if self.has_srid {
            header += 4;
        }
        self.geometries.iter().fold(header, |acc, x| acc + x.size())
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
