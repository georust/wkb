use std::io::Write;

use geo_traits::*;

use crate::error::WKBResult;
use crate::writer::{polygon_wkb_size, write_polygon};
use crate::Endianness;

/// A wrapper around an impl TriangleTrait to provide LineStringTrait and PolygonTrait
struct TriangleWrapper<'a, G: TriangleTrait<T = f64>>(&'a G);

impl<'a, G: TriangleTrait<T = f64>> LineStringTrait for &TriangleWrapper<'a, G> {
    type CoordType<'b>
        = G::CoordType<'a>
    where
        G: 'b,
        Self: 'b;

    fn num_coords(&self) -> usize {
        3
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        match i {
            0 => self.0.first(),
            1 => self.0.second(),
            2 => self.0.third(),
            _ => unreachable!(),
        }
    }
}

impl<'a, G: TriangleTrait<T = f64>> LineStringTrait for TriangleWrapper<'a, G> {
    type CoordType<'b>
        = G::CoordType<'a>
    where
        G: 'b,
        Self: 'b;

    fn num_coords(&self) -> usize {
        (&self).num_coords()
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        (&self).coord_unchecked(i)
    }
}

impl<'a, G: TriangleTrait<T = f64>> GeometryTrait for &TriangleWrapper<'a, G> {
    type T = f64;

    type PointType<'c>
        = UnimplementedPoint<f64>
    where
        Self: 'c;

    type LineStringType<'c>
        = TriangleWrapper<'a, G>
    where
        Self: 'c;

    type PolygonType<'c>
        = UnimplementedPolygon<f64>
    where
        Self: 'c;

    type MultiPointType<'c>
        = UnimplementedMultiPoint<f64>
    where
        Self: 'c;

    type MultiLineStringType<'c>
        = UnimplementedMultiLineString<f64>
    where
        Self: 'c;

    type MultiPolygonType<'c>
        = UnimplementedMultiPolygon<f64>
    where
        Self: 'c;

    type GeometryCollectionType<'c>
        = UnimplementedGeometryCollection<f64>
    where
        Self: 'c;

    type RectType<'c>
        = UnimplementedRect<f64>
    where
        Self: 'c;

    type TriangleType<'c>
        = UnimplementedTriangle<f64>
    where
        Self: 'c;

    type LineType<'c>
        = UnimplementedLine<f64>
    where
        Self: 'c;

    fn dim(&self) -> Dimensions {
        self.0.dim()
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        UnimplementedPoint<f64>,
        TriangleWrapper<'a, G>,
        UnimplementedPolygon<f64>,
        UnimplementedMultiPoint<f64>,
        UnimplementedMultiLineString<f64>,
        UnimplementedMultiPolygon<f64>,
        UnimplementedGeometryCollection<f64>,
        UnimplementedRect<f64>,
        UnimplementedTriangle<f64>,
        UnimplementedLine<f64>,
    > {
        geo_traits::GeometryType::LineString(self)
    }
}

impl<G: TriangleTrait<T = f64>> PolygonTrait for TriangleWrapper<'_, G> {
    type RingType<'b>
        = &'b TriangleWrapper<'b, G>
    where
        G: 'b,
        Self: 'b;

    fn exterior(&self) -> Option<Self::RingType<'_>> {
        Some(self)
    }

    fn num_interiors(&self) -> usize {
        0
    }

    unsafe fn interior_unchecked(&self, _i: usize) -> Self::RingType<'_> {
        unreachable!()
    }
}

impl<'a, G: TriangleTrait<T = f64>> GeometryTrait for TriangleWrapper<'a, G> {
    type T = f64;

    type PointType<'b>
        = UnimplementedPoint<f64>
    where
        Self: 'b;

    type LineStringType<'b>
        = UnimplementedLineString<f64>
    where
        Self: 'b;

    type PolygonType<'b>
        = TriangleWrapper<'a, G>
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

    type TriangleType<'b>
        = UnimplementedTriangle<f64>
    where
        Self: 'b;

    type LineType<'b>
        = UnimplementedLine<f64>
    where
        Self: 'b;

    fn dim(&self) -> Dimensions {
        self.0.dim()
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        UnimplementedPoint<f64>,
        UnimplementedLineString<f64>,
        TriangleWrapper<'a, G>,
        UnimplementedMultiPoint<f64>,
        UnimplementedMultiLineString<f64>,
        UnimplementedMultiPolygon<f64>,
        UnimplementedGeometryCollection<f64>,
        UnimplementedRect<f64>,
        UnimplementedTriangle<f64>,
        UnimplementedLine<f64>,
    > {
        geo_traits::GeometryType::Polygon(self)
    }
}

/// The number of bytes this Triangle will take up when encoded as WKB
pub fn triangle_wkb_size(geom: &impl TriangleTrait<T = f64>) -> usize {
    polygon_wkb_size(&TriangleWrapper(geom))
}

/// Write a Triangle geometry to a Writer encoded as WKB
pub fn write_triangle(
    writer: &mut impl Write,
    geom: &impl TriangleTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    write_polygon(writer, &TriangleWrapper(geom), endianness)
}
