use std::io::Cursor;

use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::linearring::LinearRing;
use crate::reader::util::{has_srid, ReadBytesExt};
use crate::reader::HEADER_BYTES;
use crate::Endianness;
use geo_traits::PolygonTrait;

/// A WKB Polygon
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
#[derive(Debug, Clone)]
pub struct Polygon<'a> {
    wkb_linear_rings: Vec<LinearRing<'a>>,
    buf: &'a [u8],
    dim: Dimension,
}

impl<'a> Polygon<'a> {
    /// Construct a new Polygon from a WKB buffer.
    ///
    /// This will parse the WKB header and extract all linear rings.
    pub(crate) fn try_new(
        buf: &'a [u8],
        byte_order: Endianness,
        dim: Dimension,
    ) -> WkbResult<Self> {
        let has_srid = has_srid(buf, byte_order)?;
        let num_rings_offset = HEADER_BYTES + if has_srid { 4 } else { 0 };

        let mut reader = Cursor::new(buf);
        reader.set_position(num_rings_offset);

        let num_rings = reader
            .read_u32(byte_order)?
            .try_into()
            .map_err(|e| WkbError::General(format!("Invalid number of rings: {}", e)))?;

        let mut ring_offset = num_rings_offset + 4;
        let mut wkb_linear_rings = Vec::with_capacity(num_rings);
        for _ in 0..num_rings {
            let ring = LinearRing::try_new(&buf[ring_offset as usize..], byte_order, dim)?;
            ring_offset += ring.size();
            wkb_linear_rings.push(ring);
        }

        Ok(Self {
            wkb_linear_rings,
            buf: &buf[0..ring_offset as usize],
            dim,
        })
    }

    /// The number of bytes in this object, including any header
    #[inline]
    pub fn size(&self) -> u64 {
        self.buf.len() as u64
    }

    /// The dimension of this Polygon
    pub fn dimension(&self) -> Dimension {
        self.dim
    }

    /// Get the underlying buffer of this Polygon
    #[inline]
    pub fn buf(&self) -> &'a [u8] {
        self.buf
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
