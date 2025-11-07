use std::io::Cursor;

use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::polygon::Polygon;
use crate::reader::util::{has_srid, ReadBytesExt};
use crate::reader::HEADER_BYTES;
use crate::Endianness;
use geo_traits::MultiPolygonTrait;

/// A WKB MultiPolygon
#[derive(Debug, Clone)]
pub struct MultiPolygon<'a> {
    /// A Polygon object for each of the internal line strings
    wkb_polygons: Vec<Polygon<'a>>,
    buf: &'a [u8],
    dim: Dimension,
}

impl<'a> MultiPolygon<'a> {
    pub(crate) fn try_new(
        buf: &'a [u8],
        byte_order: Endianness,
        dim: Dimension,
    ) -> WkbResult<Self> {
        let has_srid = has_srid(buf, byte_order)?;
        let num_polygons_offset = HEADER_BYTES + if has_srid { 4 } else { 0 };

        let mut reader = Cursor::new(buf);
        reader.set_position(num_polygons_offset);
        let num_polygons = reader
            .read_u32(byte_order)?
            .try_into()
            .map_err(|e| WkbError::General(format!("Invalid number of polygons: {}", e)))?;

        let mut polygon_offset = num_polygons_offset + 4;

        let mut wkb_polygons = Vec::with_capacity(num_polygons);
        for _ in 0..num_polygons {
            let polygon = Polygon::try_new(&buf[polygon_offset as usize..], byte_order, dim)?;
            polygon_offset += polygon.size();
            wkb_polygons.push(polygon);
        }

        Ok(Self {
            wkb_polygons,
            buf: &buf[0..polygon_offset as usize],
            dim,
        })
    }

    /// The number of bytes in this object, including any header
    #[inline]
    pub fn size(&self) -> u64 {
        self.buf.len() as u64
    }

    /// The dimension of this MultiPolygon
    pub fn dimension(&self) -> Dimension {
        self.dim
    }

    /// Get the underlying buffer of this MultiPolygon
    pub fn buf(&self) -> &'a [u8] {
        self.buf
    }
}

impl<'a> MultiPolygonTrait for MultiPolygon<'a> {
    type InnerPolygonType<'b>
        = &'b Polygon<'a>
    where
        Self: 'b;

    fn num_polygons(&self) -> usize {
        self.wkb_polygons.len()
    }

    unsafe fn polygon_unchecked(&self, i: usize) -> Self::InnerPolygonType<'_> {
        self.wkb_polygons.get_unchecked(i)
    }
}

impl<'a, 'b> MultiPolygonTrait for &'b MultiPolygon<'a> {
    type InnerPolygonType<'c>
        = &'b Polygon<'a>
    where
        Self: 'c;

    fn num_polygons(&self) -> usize {
        self.wkb_polygons.len()
    }

    unsafe fn polygon_unchecked(&self, i: usize) -> Self::InnerPolygonType<'_> {
        self.wkb_polygons.get_unchecked(i)
    }
}
