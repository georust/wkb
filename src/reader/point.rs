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
    dim: Dimension,
    is_empty: bool,
    has_srid: bool,
}

impl<'a> Point<'a> {
    pub(crate) fn try_new(
        buf: &'a [u8],
        byte_order: Endianness,
        offset: u64,
        dim: Dimension,
    ) -> WkbResult<Self> {
        let has_srid = has_srid(buf, byte_order, offset)?;

        // The space of the byte order + geometry type
        let mut offset = offset + 5;
        if has_srid {
            // Skip SRID bytes if they exist
            offset += 4;
        }

        let expected_end = offset as usize + dim.size() * 8;
        if buf.len() < expected_end {
            return Self::handle_invalid_buffer_length(offset, expected_end, buf.len());
        }

        let coord = Coord::new(buf, byte_order, offset, dim);
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
            dim,
            is_empty,
            has_srid,
        })
    }

    #[cold]
    fn handle_invalid_buffer_length(
        offset: u64,
        expected_end: usize,
        buf_len: usize,
    ) -> WkbResult<Self> {
        Err(WkbError::General(format!(
            "Invalid buffer length for Point: geometry starting at offset {} would end at byte {}, but buffer length is {}.",
            offset, expected_end, buf_len
        )))
    }

    /// The number of bytes in this object, including any header
    ///
    /// Note that this is not the same as the length of the underlying buffer
    pub fn size(&self) -> u64 {
        // - 1: byteOrder
        // - 4: wkbType
        // - dim size * 8: two f64s
        let mut header = 1 + 4;
        if self.has_srid {
            header += 4;
        }
        header + (self.dim.size() as u64 * 8)
    }

    pub fn dimension(&self) -> Dimension {
        self.dim
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
