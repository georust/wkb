use std::io::Cursor;

use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::point::Point;
use crate::reader::util::{has_srid, ReadBytesExt};
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
    dim: Dimension,
    has_srid: bool,
}

impl<'a> MultiPoint<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(buf: &'a [u8], byte_order: Endianness, dim: Dimension) -> Self {
        Self::try_new(buf, byte_order, dim).unwrap()
    }

    pub(crate) fn try_new(
        buf: &'a [u8],
        byte_order: Endianness,
        dim: Dimension,
    ) -> WkbResult<Self> {
        let mut offset = 0;
        let has_srid = has_srid(buf, byte_order, offset)?;
        if has_srid {
            offset += 4;
        }

        let mut reader = Cursor::new(buf);
        // Set reader to after 1-byte byteOrder and 4-byte wkbType
        reader.set_position(1 + 4 + offset);
        let num_points = reader
            .read_u32(byte_order)?
            .try_into()
            .map_err(|e| WkbError::General(format!("Invalid number of points: {}", e)))?;

        let multipoint = Self {
            buf,
            byte_order,
            num_points,
            dim,
            has_srid,
        };

        let end_offset = multipoint.point_offset(num_points as u64);
        if end_offset > buf.len() as u64 {
            return Self::handle_invalid_buffer_length(end_offset, buf.len());
        }

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
    ///
    /// Note that this is not the same as the length of the underlying buffer
    pub fn size(&self) -> u64 {
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numPoints
        // - Point::size() * self.num_points: the size of each Point for each point
        let mut header = 1 + 4 + 4;
        if self.has_srid {
            header += 4;
        }
        header + ((1 + 4 + (self.dim.size() as u64 * 8)) * self.num_points as u64)
    }

    /// The offset into this buffer of any given Point
    pub fn point_offset(&self, i: u64) -> u64 {
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numPoints
        let mut header = 1 + 4 + 4;
        if self.has_srid {
            header += 4;
        }
        header + ((1 + 4 + (self.dim.size() as u64 * 8)) * i)
    }

    pub fn dimension(&self) -> Dimension {
        self.dim
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
        Point::new(
            self.buf,
            self.byte_order,
            self.point_offset(i as u64),
            self.dim,
        )
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
        Point::new(
            self.buf,
            self.byte_order,
            self.point_offset(i as u64),
            self.dim,
        )
    }
}
