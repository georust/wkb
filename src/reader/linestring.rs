use std::io::Cursor;

use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::coord::Coord;
use crate::reader::util::{has_srid, ReadBytesExt};
use crate::reader::HEADER_BYTES;
use crate::Endianness;
use geo_traits::LineStringTrait;

/// A WKB LineString
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
#[derive(Debug, Clone, Copy)]
pub struct LineString<'a> {
    buf: &'a [u8],
    byte_order: Endianness,

    /// The number of points in this LineString WKB
    num_points: usize,

    /// The offset into the buffer where the first coord is located
    coord_offset: u64,
    dim: Dimension,
}

impl<'a> LineString<'a> {
    /// Construct a new LineString from a WKB buffer.
    ///
    /// This will parse the WKB header and validate the buffer length.
    pub(crate) fn try_new(
        buf: &'a [u8],
        byte_order: Endianness,
        dim: Dimension,
    ) -> WkbResult<Self> {
        let has_srid = has_srid(buf, byte_order)?;

        let num_points_offset = HEADER_BYTES + if has_srid { 4 } else { 0 };
        let mut reader = Cursor::new(buf);
        reader.set_position(num_points_offset);
        let num_points = reader
            .read_u32(byte_order)?
            .try_into()
            .map_err(|e| WkbError::General(format!("Invalid number of points: {}", e)))?;

        let coord_offset = num_points_offset + 4; // Skip the 4-byte num_points field

        let mut linestring = Self {
            buf,
            byte_order,
            num_points,
            coord_offset,
            dim,
        };

        let expected_end_abs = linestring.coord_offset(num_points as u64);
        if expected_end_abs > buf.len() as u64 {
            return Self::handle_invalid_buffer_length(expected_end_abs, buf.len());
        }

        linestring.buf = &linestring.buf[0..expected_end_abs as usize];

        Ok(linestring)
    }

    #[cold]
    fn handle_invalid_buffer_length(expected_end_abs: u64, buf_len: usize) -> WkbResult<Self> {
        Err(WkbError::General(format!(
            "Invalid buffer length for LineString: geometry would end at byte {}, but buffer length is {}.",
            expected_end_abs, buf_len
        )))
    }

    /// The number of bytes in this object, including any header
    #[inline]
    pub fn size(&self) -> u64 {
        self.buf.len() as u64
    }

    /// The offset into this buffer of any given coordinate
    pub fn coord_offset(&self, i: u64) -> u64 {
        self.coord_offset + (self.dim.size() as u64 * 8 * i)
    }

    /// The dimension of this LineString
    #[inline]
    pub fn dimension(&self) -> Dimension {
        self.dim
    }

    /// The slice of bytes containing the coordinates of this LineString. The byte order
    /// of LineString can be obtained by calling [LineString::byte_order].
    #[inline]
    pub fn coords_slice(&self) -> &'a [u8] {
        let start = self.coord_offset(0) as usize;
        let end = self.coord_offset(self.num_points as u64) as usize;
        &self.buf[start..end]
    }

    /// Get the byte order of WKB LineString
    #[inline]
    pub fn byte_order(&self) -> Endianness {
        self.byte_order
    }

    /// Get the underlying buffer of this LineString
    #[inline]
    pub fn buf(&self) -> &'a [u8] {
        self.buf
    }
}

impl<'a> LineStringTrait for LineString<'a> {
    type CoordType<'b>
        = Coord<'a>
    where
        Self: 'b;

    #[inline]
    fn num_coords(&self) -> usize {
        self.num_points
    }

    #[inline]
    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        let coord_offset = self.coord_offset(i as u64) as usize;
        let buf = &self.buf[coord_offset..];
        Coord::new(buf, self.byte_order, self.dim)
    }
}

impl<'a> LineStringTrait for &LineString<'a> {
    type CoordType<'b>
        = Coord<'a>
    where
        Self: 'b;

    #[inline]
    fn num_coords(&self) -> usize {
        self.num_points
    }

    #[inline]
    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        let coord_offset = self.coord_offset(i as u64) as usize;
        let buf = &self.buf[coord_offset..];
        Coord::new(buf, self.byte_order, self.dim)
    }
}
