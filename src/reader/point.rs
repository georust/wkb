use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::coord::Coord;
use crate::reader::util::has_srid;
use crate::Endianness;
use geo_traits::{CoordTrait, PointTrait};

/// A WKB Point.
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
///
/// See page 66 of <https://portal.ogc.org/files/?artifact_id=25355>.
#[derive(Debug, Clone, Copy)]
pub struct Point<'a> {
    /// The coordinate inside this Point
    coord: Coord<'a>,
    buf: &'a [u8],
    dim: Dimension,
    is_empty: bool,
}

impl<'a> Point<'a> {
    pub(crate) fn new(buf: &'a [u8], byte_order: Endianness, dim: Dimension) -> Self {
        Self::try_new(buf, byte_order, dim).unwrap()
    }

    pub(crate) fn try_new(
        buf: &'a [u8],
        byte_order: Endianness,
        dim: Dimension,
    ) -> WkbResult<Self> {
        let has_srid = has_srid(buf, byte_order)?;

        // The space of the byte order + geometry type
        let mut offset = 5;
        if has_srid {
            // Skip SRID bytes if they exist
            offset += 4;
        }

        let expected_end = offset as usize + dim.size() * 8;
        if buf.len() < expected_end {
            return Self::handle_invalid_buffer_length(expected_end, buf.len());
        }

        let coord = Coord::new(&buf[offset as usize..expected_end], byte_order, dim);
        let is_empty = (0..coord.dim().size()).all(|coord_dim| {
            {
                // Safety:
                // We just checked the number of dimensions, and coord_dim is less than
                // coord.dim().size()
                unsafe { coord.nth_unchecked(coord_dim) }
            }
            .is_nan()
        });
        Ok(Self {
            coord,
            buf: &buf[0..expected_end],
            dim,
            is_empty,
        })
    }

    #[cold]
    fn handle_invalid_buffer_length(expected_end: usize, buf_len: usize) -> WkbResult<Self> {
        Err(WkbError::General(format!(
            "Invalid buffer length for Point: geometry would end at byte {}, but buffer length is {}.",
            expected_end, buf_len
        )))
    }

    /// The number of bytes in this object, including any header
    #[inline]
    pub fn size(&self) -> u64 {
        self.buf.len() as u64
    }

    /// The dimension of this Point
    #[inline]
    pub fn dimension(&self) -> Dimension {
        self.dim
    }

    /// Whether this Point is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    /// Get the byte order of WKB point
    #[inline]
    pub fn byte_order(&self) -> Endianness {
        self.coord.byte_order()
    }

    /// Get the slice of bytes containing the coordinate. The byte order
    /// of coordinate can be obtained by calling [Point::byte_order].
    #[inline]
    pub fn coord_slice(&self) -> &'a [u8] {
        self.coord.coord_slice()
    }

    /// Get the underlying buffer of this Point
    #[inline]
    pub fn buf(&self) -> &'a [u8] {
        self.buf
    }
}

impl<'a> PointTrait for Point<'a> {
    type CoordType<'b>
        = Coord<'a>
    where
        Self: 'b;

    fn coord(&self) -> Option<Self::CoordType<'_>> {
        if self.is_empty {
            None
        } else {
            Some(self.coord)
        }
    }
}

impl<'a> PointTrait for &Point<'a> {
    type CoordType<'b>
        = Coord<'a>
    where
        Self: 'b;

    fn coord(&self) -> Option<Self::CoordType<'_>> {
        if self.is_empty {
            None
        } else {
            Some(self.coord)
        }
    }
}
