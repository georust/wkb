use std::io::Cursor;

use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::coord::Coord;
use crate::reader::util::{has_srid, ReadBytesExt};
use crate::Endianness;
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
    dim: Dimension,
    has_srid: bool,
}

impl<'a> LineString<'a> {
    /// Construct a new LineString from a WKB buffer.
    ///
    /// This will parse the WKB header and validate the buffer length.
    pub(crate) fn try_new(
        buf: &'a [u8],
        byte_order: Endianness,
        mut offset: u64,
        dim: Dimension,
    ) -> WkbResult<Self> {
        let has_srid = has_srid(buf, byte_order, offset)?;
        if has_srid {
            offset += 4;
        }

        let mut reader = Cursor::new(buf);
        reader.set_position(HEADER_BYTES + offset);
        let num_points = reader
            .read_u32(byte_order)?
            .try_into()
            .map_err(|e| WkbError::General(format!("Invalid number of points: {}", e)))?;

        let linestring = Self {
            buf,
            byte_order,
            num_points,
            offset,
            dim,
            has_srid,
        };

        let expected_end_abs = linestring.coord_offset(num_points as u64);
        if expected_end_abs > buf.len() as u64 {
            return Self::handle_invalid_buffer_length(
                linestring.offset,
                expected_end_abs,
                buf.len(),
            );
        }

        Ok(linestring)
    }

    #[cold]
    fn handle_invalid_buffer_length(
        offset: u64,
        expected_end_abs: u64,
        buf_len: usize,
    ) -> WkbResult<Self> {
        Err(WkbError::General(format!(
            "Invalid buffer length for LineString: geometry starting at offset {} would end at byte {}, but buffer length is {}.",
            offset, expected_end_abs, buf_len
        )))
    }

    /// The number of bytes in this object, including any header
    ///
    /// Note that this is not the same as the length of the underlying buffer
    pub fn size(&self) -> u64 {
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numPoints
        // - 2 * 8 * self.num_points: two f64s for each coordinate
        let mut header = 1 + 4 + 4;
        if self.has_srid {
            header += 4;
        }
        header + (self.dim.size() as u64 * 8 * self.num_points as u64)
    }

    /// The offset into this buffer of any given coordinate
    pub fn coord_offset(&self, i: u64) -> u64 {
        self.offset + 1 + 4 + 4 + (self.dim.size() as u64 * 8 * i)
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
        let end = start + self.dim.size() * 8 * self.num_points;
        &self.buf[start..end]
    }

    /// Get the byte order of WKB LineString
    #[inline]
    pub fn byte_order(&self) -> Endianness {
        self.byte_order
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
        Coord::new(
            self.buf,
            self.byte_order,
            self.coord_offset(i as u64),
            self.dim,
        )
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
        Coord::new(
            self.buf,
            self.byte_order,
            self.coord_offset(i as u64),
            self.dim,
        )
    }
}
