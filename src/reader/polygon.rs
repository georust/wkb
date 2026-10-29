use std::io::Cursor;

use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::linearring::LinearRing;
use crate::reader::util::{bounded_capacity, has_srid, ReadBytesExt, MIN_LINEAR_RING_SIZE};
use crate::reader::HEADER_BYTES;
use crate::Endianness;
use geo_traits::PolygonTrait;

/// A WKB Polygon
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
#[derive(Debug, Clone)]
pub struct Polygon<'a> {
    exterior: Exterior<'a>,
    interiors: Box<[LinearRing<'a>]>,
    buf: &'a [u8],
}

/// The exterior ring of a [`Polygon`], or the polygon's dimension if it has no rings.
///
/// An empty polygon has no ring to read its dimension from, so `Missing` carries it instead
#[derive(Debug, Clone, Copy)]
enum Exterior<'a> {
    Missing(Dimension),
    Present(LinearRing<'a>),
}

/// Fixme: use `std::hint::cold_path` once MSRV is Rust 1.95.
#[cold]
#[inline]
fn cold_path() {}

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
        let capacity = bounded_capacity(
            num_rings,
            buf.len().saturating_sub(ring_offset as usize),
            MIN_LINEAR_RING_SIZE,
        );
        let exterior = if num_rings > 0 {
            let ring = LinearRing::try_new(&buf[ring_offset as usize..], byte_order, dim)?;
            ring_offset += ring.size();
            Exterior::Present(ring)
        } else {
            cold_path();
            Exterior::Missing(dim)
        };
        let mut interiors = Vec::with_capacity(capacity.saturating_sub(1));
        for _ in 1..num_rings {
            let ring = LinearRing::try_new(&buf[ring_offset as usize..], byte_order, dim)?;
            ring_offset += ring.size();
            interiors.push(ring);
        }

        Ok(Self {
            exterior,
            interiors: interiors.into_boxed_slice(),
            buf: &buf[0..ring_offset as usize],
        })
    }

    #[inline]
    fn exterior_ring(&self) -> Option<&LinearRing<'a>> {
        match &self.exterior {
            Exterior::Present(ring) => Some(ring),
            Exterior::Missing(_) => {
                cold_path();
                None
            }
        }
    }

    /// The number of bytes in this object, including any header
    #[inline]
    pub fn size(&self) -> u64 {
        self.buf.len() as u64
    }

    /// The dimension of this Polygon
    pub fn dimension(&self) -> Dimension {
        match &self.exterior {
            Exterior::Present(ring) => ring.dimension(),
            Exterior::Missing(dim) => {
                cold_path();
                *dim
            }
        }
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
        self.interiors.len()
    }

    fn exterior(&self) -> Option<Self::RingType<'_>> {
        self.exterior_ring()
    }

    unsafe fn interior_unchecked(&self, i: usize) -> Self::RingType<'_> {
        self.interiors.get_unchecked(i)
    }
}

impl<'a, 'b> PolygonTrait for &'b Polygon<'a> {
    type RingType<'c>
        = &'b LinearRing<'a>
    where
        Self: 'c;

    fn num_interiors(&self) -> usize {
        self.interiors.len()
    }

    fn exterior(&self) -> Option<Self::RingType<'_>> {
        self.exterior_ring()
    }

    unsafe fn interior_unchecked(&self, i: usize) -> Self::RingType<'_> {
        self.interiors.get_unchecked(i)
    }
}
