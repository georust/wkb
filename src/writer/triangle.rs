use std::io::Write;

use geo_traits::{
    GeometryTrait, LineStringTrait, PolygonTrait, TriangleTrait, UnimplementedGeometryCollection,
    UnimplementedLine, UnimplementedLineString, UnimplementedMultiLineString,
    UnimplementedMultiPoint, UnimplementedMultiPolygon, UnimplementedPoint, UnimplementedPolygon,
    UnimplementedRect, UnimplementedTriangle,
};

use crate::error::WkbResult;
use crate::writer::{polygon_wkb_size, write_polygon, WriteOptions};

/// A wrapper around an impl TriangleTrait to provide LineStringTrait and PolygonTrait
struct TriangleWrapper<'a, G: TriangleTrait<T = f64>>(&'a G);

impl<'a, G: TriangleTrait<T = f64>> LineStringTrait for &'a TriangleWrapper<'a, G> {
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

/// The number of bytes this Triangle will take up when encoded as WKB
pub fn triangle_wkb_size(geom: &impl TriangleTrait<T = f64>) -> usize {
    polygon_wkb_size(&TriangleWrapper(geom))
}

/// Write a Triangle geometry to a Writer encoded as WKB
pub fn write_triangle(
    writer: &mut impl Write,
    geom: &impl TriangleTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    write_polygon(writer, &TriangleWrapper(geom), options)
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
        = Self
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
        geo_traits::Dimensions::Xy
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'a>,
        Self::LineStringType<'a>,
        Self::PolygonType<'a>,
        Self::MultiPointType<'a>,
        Self::MultiLineStringType<'a>,
        Self::MultiPolygonType<'a>,
        Self::GeometryCollectionType<'a>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        geo_traits::GeometryType::Polygon(self)
    }
}

impl<'a, G: TriangleTrait<T = f64>> GeometryTrait for &'a TriangleWrapper<'a, G> {
    type T = f64;
    type PointType<'b>
        = UnimplementedPoint<f64>
    where
        Self: 'b;
    type LineStringType<'b>
        = Self
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
        geo_traits::Dimensions::Xy
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'a>,
        Self::LineStringType<'a>,
        Self::PolygonType<'a>,
        Self::MultiPointType<'a>,
        Self::MultiLineStringType<'a>,
        Self::MultiPolygonType<'a>,
        Self::GeometryCollectionType<'a>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        geo_traits::GeometryType::LineString(self)
    }
}
