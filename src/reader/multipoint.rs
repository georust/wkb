use std::io::Cursor;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::reader::point::WKBPoint;
use crate::Endianness;
use geo_traits::Dimensions;
use geo_traits::MultiPointTrait;

/// A WKB MultiPoint
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
#[derive(Debug, Clone, Copy)]
pub struct WKBMultiPoint<'a> {
    buf: &'a [u8],
    byte_order: Endianness,

    /// The number of points in this multi point
    num_points: usize,
    dim: Dimensions,
}

impl<'a> WKBMultiPoint<'a> {
    pub(crate) fn new(buf: &'a [u8], byte_order: Endianness, dim: Dimensions) -> Self {
        // TODO: assert WKB type?
        let mut reader = Cursor::new(buf);
        // Set reader to after 1-byte byteOrder and 4-byte wkbType
        reader.set_position(1 + 4);
        let num_points = match byte_order {
            Endianness::BigEndian => reader.read_u32::<BigEndian>().unwrap().try_into().unwrap(),
            Endianness::LittleEndian => reader
                .read_u32::<LittleEndian>()
                .unwrap()
                .try_into()
                .unwrap(),
        };

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
        // - WKBPoint::size() * self.num_points: the size of each WKBPoint for each point
        1 + 4 + 4 + ((1 + 4 + (self.dim.size() as u64 * 8)) * self.num_points as u64)
    }

    /// The offset into this buffer of any given WKBPoint
    pub fn point_offset(&self, i: u64) -> u64 {
        1 + 4 + 4 + ((1 + 4 + (self.dim.size() as u64 * 8)) * i)
    }

    pub fn dimension(&self) -> Dimensions {
        self.dim
    }
}

impl<'a> MultiPointTrait for WKBMultiPoint<'a> {
    type T = f64;
    type PointType<'b> = WKBPoint<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_points(&self) -> usize {
        self.num_points
    }

    unsafe fn point_unchecked(&self, i: usize) -> Self::PointType<'_> {
        WKBPoint::new(
            self.buf,
            self.byte_order,
            self.point_offset(i.try_into().unwrap()),
            self.dim,
        )
    }
}

impl<'a> MultiPointTrait for &'a WKBMultiPoint<'a> {
    type T = f64;
    type PointType<'b> = WKBPoint<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_points(&self) -> usize {
        self.num_points
    }

    unsafe fn point_unchecked(&self, i: usize) -> Self::PointType<'_> {
        WKBPoint::new(
            self.buf,
            self.byte_order,
            self.point_offset(i.try_into().unwrap()),
            self.dim,
        )
    }
}