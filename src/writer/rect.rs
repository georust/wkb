use std::io::Write;

use geo_traits::{
    CoordTrait, GeometryTrait, LineStringTrait, PolygonTrait, RectTrait,
    UnimplementedGeometryCollection, UnimplementedLine, UnimplementedLineString,
    UnimplementedMultiLineString, UnimplementedMultiPoint, UnimplementedMultiPolygon,
    UnimplementedPoint, UnimplementedPolygon, UnimplementedRect, UnimplementedTriangle,
};

use crate::error::WkbResult;
use crate::writer::{polygon_wkb_size, write_polygon, WriteOptions};

struct Coord2D {
    x: f64,
    y: f64,
}

impl CoordTrait for Coord2D {
    type T = f64;

    fn x(&self) -> Self::T {
        self.x
    }

    fn y(&self) -> Self::T {
        self.y
    }

    fn dim(&self) -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xy
    }

    fn nth_or_panic(&self, n: usize) -> Self::T {
        match n {
            0 => self.x,
            1 => self.y,
            _ => panic!(),
        }
    }
}

/// A wrapper around an impl RectTrait to provide LineStringTrait and PolygonTrait
struct RectWrapper<'a, G: RectTrait<T = f64>>(&'a G);

impl<'a, G: RectTrait<T = f64>> LineStringTrait for &'a RectWrapper<'a, G> {
    type CoordType<'b>
        = Coord2D
    where
        G: 'b,
        Self: 'b;

    fn num_coords(&self) -> usize {
        5
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        let min_coord = self.0.min();
        let max_coord = self.0.max();

        match i {
            0 => Coord2D {
                x: min_coord.x(),
                y: min_coord.y(),
            },
            1 => Coord2D {
                x: min_coord.x(),
                y: max_coord.y(),
            },
            2 => Coord2D {
                x: max_coord.x(),
                y: max_coord.y(),
            },
            3 => Coord2D {
                x: max_coord.x(),
                y: min_coord.y(),
            },
            4 => Coord2D {
                x: min_coord.x(),
                y: min_coord.y(),
            },
            _ => unreachable!(),
        }
    }
}

impl<G: RectTrait<T = f64>> PolygonTrait for RectWrapper<'_, G> {
    type RingType<'b>
        = &'b RectWrapper<'b, G>
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

/// The number of bytes this Rect will take up when encoded as WKB
pub fn rect_wkb_size(geom: &impl RectTrait<T = f64>) -> usize {
    polygon_wkb_size(&RectWrapper(geom))
}

/// Write a Rect geometry to a Writer encoded as WKB
pub fn write_rect(
    writer: &mut impl Write,
    geom: &impl RectTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    write_polygon(writer, &RectWrapper(geom), options)
}

impl<'a, G: RectTrait<T = f64>> GeometryTrait for RectWrapper<'a, G> {
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

impl<'a, G: RectTrait<T = f64>> GeometryTrait for &'a RectWrapper<'a, G> {
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
