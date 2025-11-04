use std::io::Cursor;

use crate::common::Dimension;
use crate::reader::util::ReadBytesExt;
use crate::Endianness;
use geo_traits::{CoordTrait, Dimensions};

const F64_WIDTH: u64 = 8;

/// A coordinate in a WKB buffer.
///
/// Note that according to the WKB specification this is called `"Point"`, which is **not** the
/// same as a WKB "framed" `Point`. In particular, a "framed" `Point` has framing that includes the
/// byte order and geometry type of the WKB buffer. In contrast, this `Coord` is the building block
/// of two to four f64 numbers that can occur within any geometry type.
///
/// See page 65 of <https://portal.ogc.org/files/?artifact_id=25355>.
#[derive(Debug, Clone, Copy)]
pub struct Coord<'a> {
    /// The underlying WKB buffer
    buf: &'a [u8],

    /// The byte order of this WKB buffer
    byte_order: Endianness,

    dim: Dimension,
}

impl<'a> Coord<'a> {
    pub(crate) fn new(buf: &'a [u8], byte_order: Endianness, dim: Dimension) -> Self {
        Self {
            buf,
            byte_order,
            dim,
        }
    }

    #[inline]
    fn get_x(&self) -> f64 {
        let mut reader = Cursor::new(self.buf);
        reader.read_f64(self.byte_order).unwrap()
    }

    #[inline]
    fn get_y(&self) -> f64 {
        let mut reader = Cursor::new(self.buf);
        reader.set_position(F64_WIDTH);
        reader.read_f64(self.byte_order).unwrap()
    }

    #[inline]
    fn get_nth_unchecked(&self, n: usize) -> f64 {
        debug_assert!(n < self.dim.size());
        let mut reader = Cursor::new(self.buf);
        reader.set_position(n as u64 * F64_WIDTH);
        reader.read_f64(self.byte_order).unwrap()
    }

    /// Get the byte order of WKB coordinate
    #[inline]
    pub fn byte_order(&self) -> Endianness {
        self.byte_order
    }

    /// Get the slice of bytes containing the coordinate. The byte order
    /// of coordinate can be obtained by calling [Coord::byte_order].
    #[inline]
    pub fn coord_slice(&self) -> &'a [u8] {
        let end = self.size() as usize;
        &self.buf[0..end]
    }

    /// Get the dimension of this coordinate
    #[inline]
    pub fn dimension(&self) -> Dimension {
        self.dim
    }

    /// The number of bytes in this object
    #[inline]
    pub fn size(&self) -> u64 {
        // A 2D Coord is just two f64s
        self.dim.size() as u64 * 8
    }
}

impl CoordTrait for Coord<'_> {
    type T = f64;

    fn dim(&self) -> Dimensions {
        self.dim.into()
    }

    #[inline]
    fn nth_or_panic(&self, n: usize) -> Self::T {
        self.get_nth_unchecked(n)
    }

    #[inline]
    fn x(&self) -> Self::T {
        self.get_x()
    }

    #[inline]
    fn y(&self) -> Self::T {
        self.get_y()
    }
}
