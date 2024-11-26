use std::io::Cursor;

use crate::common::WKBDimension;
use crate::reader::coord::Coord;
use crate::reader::util::ReadBytesExt;
use crate::Endianness;
use geo_traits::Dimensions;
use geo_traits::LineStringTrait;

const HEADER_BYTES: u64 = 5;

/// A WKB LineString
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
#[derive(Debug, Clone, Copy)]
pub struct LineString<'a> {
    buf: &'a [u8],
    byte_order: Endianness,

    /// The number of points in this LineString WKB
    num_points: usize,

    /// This offset will be 0 for a single LineString but it will be non zero for a
    /// LineString contained within a MultiLineString
    offset: u64,
    dim: WKBDimension,
}

impl<'a> LineString<'a> {
    pub fn new(buf: &'a [u8], byte_order: Endianness, offset: u64, dim: WKBDimension) -> Self {
        let mut reader = Cursor::new(buf);
        reader.set_position(HEADER_BYTES + offset);
        let num_points = reader.read_u32(byte_order).unwrap().try_into().unwrap();

        Self {
            buf,
            byte_order,
            num_points,
            offset,
            dim,
        }
    }

    /// The number of bytes in this object, including any header
    ///
    /// Note that this is not the same as the length of the underlying buffer
    pub fn size(&self) -> u64 {
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numPoints
        // - 2 * 8 * self.num_points: two f64s for each coordinate
        1 + 4 + 4 + (self.dim.size() as u64 * 8 * self.num_points as u64)
    }

    /// The offset into this buffer of any given coordinate
    pub fn coord_offset(&self, i: u64) -> u64 {
        self.offset + 1 + 4 + 4 + (self.dim.size() as u64 * 8 * i)
    }

    pub fn dimension(&self) -> WKBDimension {
        self.dim
    }
}

impl<'a> LineStringTrait for LineString<'a> {
    type T = f64;
    type CoordType<'b> = Coord<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim.into()
    }

    fn num_coords(&self) -> usize {
        self.num_points
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        Coord::new(
            self.buf,
            self.byte_order,
            self.coord_offset(i.try_into().unwrap()),
            self.dim,
        )
    }
}

impl<'a> LineStringTrait for &'a LineString<'a> {
    type T = f64;
    type CoordType<'b> = Coord<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim.into()
    }

    fn num_coords(&self) -> usize {
        self.num_points
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        Coord::new(
            self.buf,
            self.byte_order,
            self.coord_offset(i.try_into().unwrap()),
            self.dim,
        )
    }
}
