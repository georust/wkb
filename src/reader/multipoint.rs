use std::io::Cursor;

use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::point::Point;
use crate::reader::util::{has_srid, ReadBytesExt};
use crate::reader::HEADER_BYTES;
use crate::Endianness;
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

    /// The offset into the buffer where the first point is located
    points_offset: u64,

    dim: Dimension,
}

impl<'a> MultiPoint<'a> {
    pub(crate) fn try_new(
        buf: &'a [u8],
        byte_order: Endianness,
        dim: Dimension,
    ) -> WkbResult<Self> {
        let has_srid = has_srid(buf, byte_order)?;
        let num_points_offset = HEADER_BYTES + if has_srid { 4 } else { 0 };

        let mut reader = Cursor::new(buf);
        // Set reader to after 1-byte byteOrder and 4-byte wkbType
        reader.set_position(num_points_offset);
        let num_points = reader
            .read_u32(byte_order)?
            .try_into()
            .map_err(|e| WkbError::General(format!("Invalid number of points: {}", e)))?;

        let points_offset = num_points_offset + 4;
        let mut multipoint = Self {
            buf,
            byte_order,
            num_points,
            points_offset,
            dim,
        };

        let end_offset = multipoint.point_offset(num_points as u64);
        if end_offset > buf.len() as u64 {
            return Self::handle_invalid_buffer_length(end_offset, buf.len());
        }

        multipoint.buf = &buf[0..end_offset as usize];

        Ok(multipoint)
    }

    #[cold]
    fn handle_invalid_buffer_length(expected_end_abs: u64, buf_len: usize) -> WkbResult<Self> {
        Err(WkbError::General(format!(
            "Invalid buffer length for MultiPoint: geometry would end at byte {}, but buffer length is {}.",
            expected_end_abs, buf_len
        )))
    }

    /// The number of bytes in this object, including any header
    #[inline]
    pub fn size(&self) -> u64 {
        self.buf.len() as u64
    }

    /// The offset into this buffer of any given Point
    pub fn point_offset(&self, i: u64) -> u64 {
        self.points_offset + ((HEADER_BYTES + (self.dim.size() as u64 * 8)) * i)
    }

    /// The dimension of this MultiPoint
    pub fn dimension(&self) -> Dimension {
        self.dim
    }

    /// Get the underlying buffer of this MultiPoint
    pub fn buf(&self) -> &'a [u8] {
        self.buf
    }
}

impl<'a> MultiPointTrait for MultiPoint<'a> {
    type InnerPointType<'b>
        = Point<'a>
    where
        Self: 'b;

    fn num_points(&self) -> usize {
        self.num_points
    }

    unsafe fn point_unchecked(&self, i: usize) -> Self::InnerPointType<'_> {
        let offset = self.point_offset(i as u64);
        Point::new(&self.buf[offset as usize..], self.byte_order, self.dim)
    }
}

impl<'a> MultiPointTrait for &MultiPoint<'a> {
    type InnerPointType<'b>
        = Point<'a>
    where
        Self: 'b;

    fn num_points(&self) -> usize {
        self.num_points
    }

    unsafe fn point_unchecked(&self, i: usize) -> Self::InnerPointType<'_> {
        let offset = self.point_offset(i as u64);
        Point::new(&self.buf[offset as usize..], self.byte_order, self.dim)
    }
}
