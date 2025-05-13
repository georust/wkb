use std::io::Cursor;

use crate::common::Dimension;
use crate::reader::polygon::Polygon;
use crate::reader::util::{has_srid, ReadBytesExt};
use crate::Endianness;
use geo_traits::Dimensions;
use geo_traits::MultiPolygonTrait;

/// skip endianness and wkb type
const HEADER_BYTES: u64 = 5;

/// A WKB MultiPolygon
#[derive(Debug, Clone)]
pub struct MultiPolygon<'a> {
    /// A Polygon object for each of the internal line strings
    wkb_polygons: Vec<Polygon<'a>>,

    dim: Dimension,
    has_srid: bool,
}

impl<'a> MultiPolygon<'a> {
    pub(crate) fn new(buf: &'a [u8], byte_order: Endianness, dim: Dimension) -> Self {
        let mut offset = 0;
        let has_srid = has_srid(buf, byte_order, offset);
        if has_srid {
            offset += 4;
        }

        let mut reader = Cursor::new(buf);
        reader.set_position(HEADER_BYTES + offset);
        let num_polygons = reader.read_u32(byte_order).unwrap().try_into().unwrap();

        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numLineStrings
        let mut polygon_offset = 1 + 4 + 4;
        if has_srid {
            polygon_offset += 4;
        }

        let mut wkb_polygons = Vec::with_capacity(num_polygons);
        for _ in 0..num_polygons {
            let polygon = Polygon::new(buf, byte_order, polygon_offset, dim);
            polygon_offset += polygon.size();
            wkb_polygons.push(polygon);
        }

        Self {
            wkb_polygons,
            dim,
            has_srid,
        }
    }

    /// The number of bytes in this object, including any header
    ///
    /// Note that this is not the same as the length of the underlying buffer
    pub fn size(&self) -> u64 {
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numPolygons
        let mut header = 1 + 4 + 4;
        if self.has_srid {
            header += 4;
        }
        self.wkb_polygons
            .iter()
            .fold(header, |acc, x| acc + x.size())
    }

    pub fn dimension(&self) -> Dimension {
        self.dim
    }
}

impl<'a> MultiPolygonTrait for MultiPolygon<'a> {
    type T = f64;
    type PolygonType<'b>
        = &'b Polygon<'a>
    where
        Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim.into()
    }

    fn num_polygons(&self) -> usize {
        self.wkb_polygons.len()
    }

    unsafe fn polygon_unchecked(&self, i: usize) -> Self::PolygonType<'_> {
        self.wkb_polygons.get_unchecked(i)
    }
}

impl<'a, 'b> MultiPolygonTrait for &'b MultiPolygon<'a> {
    type T = f64;
    type PolygonType<'c>
        = &'b Polygon<'a>
    where
        Self: 'c;

    fn dim(&self) -> Dimensions {
        self.dim.into()
    }

    fn num_polygons(&self) -> usize {
        self.wkb_polygons.len()
    }

    unsafe fn polygon_unchecked(&self, i: usize) -> Self::PolygonType<'_> {
        self.wkb_polygons.get_unchecked(i)
    }
}
