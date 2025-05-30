use std::io::Write;

use geo_traits::{
    GeometryTrait, LineStringTrait, LineTrait, UnimplementedGeometryCollection, UnimplementedLine,
    UnimplementedMultiLineString, UnimplementedMultiPoint, UnimplementedMultiPolygon,
    UnimplementedPoint, UnimplementedPolygon, UnimplementedRect, UnimplementedTriangle,
};

use crate::error::WkbResult;
use crate::writer::{line_string_wkb_size, write_line_string, WriteOptions};

/// A wrapper around an impl LineTrait to provide LineStringTrait
struct LineWrapper<'a, G: LineTrait<T = f64>>(&'a G);

impl<'a, G: LineTrait<T = f64>> LineStringTrait for LineWrapper<'a, G> {
    type CoordType<'b>
        = G::CoordType<'a>
    where
        G: 'b,
        Self: 'b;

    fn num_coords(&self) -> usize {
        2
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        match i {
            0 => self.0.start(),
            1 => self.0.end(),
            _ => unreachable!(),
        }
    }
}

/// The number of bytes this Line will take up when encoded as WKB
pub fn line_wkb_size(geom: &impl LineTrait<T = f64>) -> usize {
    line_string_wkb_size(&LineWrapper(geom))
}

/// Write a Line geometry to a Writer encoded as WKB
pub fn write_line(
    writer: &mut impl Write,
    geom: &impl LineTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    write_line_string(writer, &LineWrapper(geom), options)
}

impl<G: LineTrait<T = f64>> GeometryTrait for LineWrapper<'_, G> {
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
        self.0.dim()
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
