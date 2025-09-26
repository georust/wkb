use std::io::Cursor;

use geo_traits::{
    GeometryTrait, LineStringTrait, UnimplementedGeometryCollection, UnimplementedLine,
    UnimplementedMultiLineString, UnimplementedMultiPoint, UnimplementedMultiPolygon,
    UnimplementedPoint, UnimplementedPolygon, UnimplementedRect, UnimplementedTriangle,
};

use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::coord::Coord;
use crate::reader::util::ReadBytesExt;
use crate::Endianness;

/// A linear ring in a WKB buffer.
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
///
/// See page 65 of <https://portal.ogc.org/files/?artifact_id=25355>.
#[derive(Debug, Clone, Copy)]
pub struct LinearRing<'a> {
    /// The underlying WKB buffer
    buf: &'a [u8],

    /// The byte order of this WKB buffer
    byte_order: Endianness,

    /// The offset into the buffer where this linear ring is located
    ///
    /// Note that this does not have to be immediately after the WKB header! For a `Point`, the
    /// `Point` is immediately after the header, but the `Point` also appears in other geometry
    /// types. I.e. the `LineString` has a header, then the number of points, then a sequence of
    /// `Point` objects.
    offset: u64,

    /// The number of points in this linear ring
    num_points: usize,

    dim: Dimension,
}

impl<'a> LinearRing<'a> {
    /// Construct a new LinearRing from a WKB buffer.
    ///
    /// # Panics
    ///
    /// This will panic if the WKB buffer is invalid. For fallible parsing, use
    /// [`try_new`](Self::try_new) instead.
    pub fn new(buf: &'a [u8], byte_order: Endianness, offset: u64, dim: Dimension) -> Self {
        Self::try_new(buf, byte_order, offset, dim).unwrap()
    }

    /// Construct a new LinearRing from a WKB buffer.
    ///
    /// This will parse the number of points and validate the buffer length.
    pub fn try_new(
        buf: &'a [u8],
        byte_order: Endianness,
        offset: u64,
        dim: Dimension,
    ) -> WkbResult<Self> {
        let mut reader = Cursor::new(buf);
        reader.set_position(offset);
        let num_points = reader
            .read_u32(byte_order)?
            .try_into()
            .map_err(|e| WkbError::General(format!("Invalid number of points: {}", e)))?;

        let ring = Self {
            buf,
            byte_order,
            offset,
            num_points,
            dim,
        };

        // `offset` is the start of the num_points field. `ring.size()` is `4 (for num_points) + coord_data`.
        let expected_end_abs = offset + ring.size();
        if expected_end_abs > buf.len() as u64 {
            return Self::handle_invalid_buffer_length(offset, expected_end_abs, buf.len());
        }

        Ok(ring)
    }

    #[cold]
    fn handle_invalid_buffer_length(
        offset: u64,
        expected_end_abs: u64,
        buf_len: usize,
    ) -> WkbResult<Self> {
        Err(WkbError::General(format!(
            "Invalid buffer length for LinearRing: data starting at offset {} would end at byte {}, but buffer length is {}.",
            offset, expected_end_abs, buf_len
        )))
    }

    /// The number of bytes in this object, including any header
    ///
    /// Note that this is not the same as the length of the underlying buffer
    pub fn size(&self) -> u64 {
        // - 4: numPoints
        // - 2 * 8 * self.num_points: two f64s for each coordinate
        4 + (self.dim.size() as u64 * 8 * self.num_points as u64)
    }

    /// The offset into this buffer of any given coordinate
    #[inline]
    pub fn coord_offset(&self, i: u64) -> u64 {
        self.offset + 4 + (self.dim.size() as u64 * 8 * i)
    }

    /// The dimension of this LinearRing
    #[inline]
    pub fn dimension(&self) -> Dimension {
        self.dim
    }

    /// The slice of bytes containing the coordinates of this LinearRing. The byte order
    /// of LinearRing can be obtained by calling [LinearRing::byte_order].
    #[inline]
    pub fn coords_slice(&self) -> &'a [u8] {
        let start = self.coord_offset(0) as usize;
        let end = start + self.dim.size() * 8 * self.num_points;
        &self.buf[start..end]
    }

    /// Get the byte order of WKB LinearRing
    #[inline]
    pub fn byte_order(&self) -> Endianness {
        self.byte_order
    }
}

impl<'a> LineStringTrait for LinearRing<'a> {
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

impl<'a> LineStringTrait for &LinearRing<'a> {
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

impl<'a> GeometryTrait for LinearRing<'a> {
    type T = f64;
    type PointType<'b>
        = UnimplementedPoint<f64>
    where
        Self: 'b;
    type LineStringType<'b>
        = LinearRing<'a>
    where
        Self: 'b;
    type PolygonType<'b>
        = UnimplementedPolygon<f64>
    where
        Self: 'b;
    type MultiPointType<'b>
        = UnimplementedMultiPoint<f64>
    where
        Self: 'b;
    type MultiLineStringType<'b>
        = UnimplementedMultiLineString<f64>
    where
        Self: 'b;
    type MultiPolygonType<'b>
        = UnimplementedMultiPolygon<f64>
    where
        Self: 'b;
    type GeometryCollectionType<'b>
        = UnimplementedGeometryCollection<f64>
    where
        Self: 'b;
    type RectType<'b>
        = UnimplementedRect<f64>
    where
        Self: 'b;
    type LineType<'b>
        = UnimplementedLine<f64>
    where
        Self: 'b;
    type TriangleType<'b>
        = UnimplementedTriangle<f64>
    where
        Self: 'b;

    fn dim(&self) -> geo_traits::Dimensions {
        self.dimension().into()
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        geo_traits::GeometryType::LineString(self)
    }
}

impl<'a> GeometryTrait for &LinearRing<'a> {
    type T = f64;
    type PointType<'b>
        = UnimplementedPoint<f64>
    where
        Self: 'b;
    type LineStringType<'b>
        = LinearRing<'a>
    where
        Self: 'b;
    type PolygonType<'b>
        = UnimplementedPolygon<f64>
    where
        Self: 'b;
    type MultiPointType<'b>
        = UnimplementedMultiPoint<f64>
    where
        Self: 'b;
    type MultiLineStringType<'b>
        = UnimplementedMultiLineString<f64>
    where
        Self: 'b;
    type MultiPolygonType<'b>
        = UnimplementedMultiPolygon<f64>
    where
        Self: 'b;
    type GeometryCollectionType<'b>
        = UnimplementedGeometryCollection<f64>
    where
        Self: 'b;
    type RectType<'b>
        = UnimplementedRect<f64>
    where
        Self: 'b;
    type LineType<'b>
        = UnimplementedLine<f64>
    where
        Self: 'b;
    type TriangleType<'b>
        = UnimplementedTriangle<f64>
    where
        Self: 'b;

    fn dim(&self) -> geo_traits::Dimensions {
        self.dimension().into()
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        geo_traits::GeometryType::LineString(self)
    }
}
