use std::io::Cursor;

use crate::common::WKBDimension;
use crate::reader::point::Point;
use crate::reader::util::ReadBytesExt;
use crate::Endianness;
use geo_traits::Dimensions;
use geo_traits::MultiPointTrait;

/// A WKB MultiPoint
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
#[derive(Debug, Clone, Copy)]
pub struct MultiPoint<'a> {
    buf: &'a [u8],
    byte_order: Endianness,

    /// The number of points in this multi point
    num_points: usize,
    dim: WKBDimension,
}

impl<'a> MultiPoint<'a> {
    pub(crate) fn new(buf: &'a [u8], byte_order: Endianness, dim: WKBDimension) -> Self {
        // TODO: assert WKB type?
        let mut reader = Cursor::new(buf);
        // Set reader to after 1-byte byteOrder and 4-byte wkbType
        reader.set_position(1 + 4);
        let num_points = reader.read_u32(byte_order).unwrap().try_into().unwrap();

        Self {
            buf,
            byte_order,
            num_points,
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
        // - Point::size() * self.num_points: the size of each Point for each point
        1 + 4 + 4 + ((1 + 4 + (self.dim.size() as u64 * 8)) * self.num_points as u64)
    }

    /// The offset into this buffer of any given Point
    pub fn point_offset(&self, i: u64) -> u64 {
        1 + 4 + 4 + ((1 + 4 + (self.dim.size() as u64 * 8)) * i)
    }

    pub fn dimension(&self) -> WKBDimension {
        self.dim
    }
}

impl<'a> MultiPointTrait for MultiPoint<'a> {
    type T = f64;
    type PointType<'b> = Point<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim.into()
    }

    fn num_points(&self) -> usize {
        self.num_points
    }

    unsafe fn point_unchecked(&self, i: usize) -> Self::PointType<'_> {
        Point::new(
            self.buf,
            self.byte_order,
            self.point_offset(i.try_into().unwrap()),
            self.dim,
        )
    }
}

impl<'a> MultiPointTrait for &'a MultiPoint<'a> {
    type T = f64;
    type PointType<'b> = Point<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim.into()
    }

    fn num_points(&self) -> usize {
        self.num_points
    }

    unsafe fn point_unchecked(&self, i: usize) -> Self::PointType<'_> {
        Point::new(
            self.buf,
            self.byte_order,
            self.point_offset(i.try_into().unwrap()),
            self.dim,
        )
    }
}
