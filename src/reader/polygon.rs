use std::io::Cursor;

use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::linearring::LinearRing;
use crate::reader::util::{has_srid, ReadBytesExt};
use crate::Endianness;
use geo_traits::PolygonTrait;

/// skip endianness and wkb type
const HEADER_BYTES: u64 = 5;

/// A WKB Polygon
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
#[derive(Debug, Clone)]
pub struct Polygon<'a> {
    wkb_linear_rings: Vec<LinearRing<'a>>,
    dim: Dimension,
    has_srid: bool,
}

impl<'a> Polygon<'a> {
    /// Construct a new Polygon from a WKB buffer.
    ///
    /// # Panics
    ///
    /// This will panic if the WKB buffer is invalid. For fallible parsing, use
    /// [`try_new`](Self::try_new) instead.
    pub fn new(buf: &'a [u8], byte_order: Endianness, offset: u64, dim: Dimension) -> Self {
        Self::try_new(buf, byte_order, offset, dim).unwrap()
    }

    /// Construct a new Polygon from a WKB buffer.
    ///
    /// This will parse the WKB header and extract all linear rings.
    pub fn try_new(
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

        let num_rings = reader
            .read_u32(byte_order)?
            .try_into()
            .map_err(|e| WkbError::General(format!("Invalid number of rings: {}", e)))?;

        // - existing offset into buffer
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numLineStrings
        let mut ring_offset = offset + 1 + 4 + 4;
        let mut wkb_linear_rings = Vec::with_capacity(num_rings);
        for _ in 0..num_rings {
            let polygon = LinearRing::try_new(buf, byte_order, ring_offset, dim)?;
            wkb_linear_rings.push(polygon);
            ring_offset += polygon.size();
        }

        Ok(Self {
            wkb_linear_rings,
            dim,
            has_srid,
        })
    }

    /// The number of bytes in this object, including any header
    ///
    /// Note that this is not the same as the length of the underlying buffer
    pub fn size(&self) -> u64 {
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numPoints
        // - size of each linear ring
        let mut header = 1 + 4 + 4;
        if self.has_srid {
            header += 4;
        }

        self.wkb_linear_rings
            .iter()
            .fold(header, |acc, ring| acc + ring.size())
    }

    /// The dimension of this Polygon
    pub fn dimension(&self) -> Dimension {
        self.dim
    }
}

impl<'a> PolygonTrait for Polygon<'a> {
    type RingType<'b>
        = &'b LinearRing<'a>
    where
        Self: 'b;

    fn num_interiors(&self) -> usize {
        // Support an empty polygon with no rings
        if self.wkb_linear_rings.is_empty() {
            0
        } else {
            self.wkb_linear_rings.len() - 1
        }
    }

    fn exterior(&self) -> Option<Self::RingType<'_>> {
        if self.wkb_linear_rings.is_empty() {
            None
        } else {
            Some(&self.wkb_linear_rings[0])
        }
    }

    unsafe fn interior_unchecked(&self, i: usize) -> Self::RingType<'_> {
        self.wkb_linear_rings.get_unchecked(i + 1)
    }
}

impl<'a, 'b> PolygonTrait for &'b Polygon<'a> {
    type RingType<'c>
        = &'b LinearRing<'a>
    where
        Self: 'c;

    fn num_interiors(&self) -> usize {
        // Support an empty polygon with no rings
        if self.wkb_linear_rings.is_empty() {
            0
        } else {
            self.wkb_linear_rings.len() - 1
        }
    }

    fn exterior(&self) -> Option<Self::RingType<'_>> {
        if self.wkb_linear_rings.is_empty() {
            None
        } else {
            Some(&self.wkb_linear_rings[0])
        }
    }

    unsafe fn interior_unchecked(&self, i: usize) -> Self::RingType<'_> {
        self.wkb_linear_rings.get_unchecked(i + 1)
    }
}
