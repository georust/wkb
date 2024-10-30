use std::io::Cursor;

use byteorder::ReadBytesExt;

use crate::common::WKBType;
use crate::error::WKBResult;
use crate::reader::{
    WKBGeometryCollection, WKBLineString, WKBMultiLineString, WKBMultiPoint, WKBMultiPolygon,
    WKBPoint, WKBPolygon,
};
use geo_traits::{
    Dimensions, GeometryTrait, UnimplementedLine, UnimplementedRect, UnimplementedTriangle,
};

#[derive(Debug, Clone)]
pub enum WKBGeometry<'a> {
    Point(WKBPoint<'a>),
    LineString(WKBLineString<'a>),
    Polygon(WKBPolygon<'a>),
    MultiPoint(WKBMultiPoint<'a>),
    MultiLineString(WKBMultiLineString<'a>),
    MultiPolygon(WKBMultiPolygon<'a>),
    GeometryCollection(WKBGeometryCollection<'a>),
}

impl<'a> WKBGeometry<'a> {
    pub fn try_new(buf: &'a [u8]) -> WKBResult<Self> {
        let mut reader = Cursor::new(buf);
        let byte_order = reader.read_u8().unwrap();
        let wkb_type = WKBType::from_buffer(buf)?;

        use Dimensions::*;

        let out = match wkb_type {
            WKBType::Point => WKBGeometry::Point(WKBPoint::new(buf, byte_order.into(), 0, Xy)),
            WKBType::LineString => {
                WKBGeometry::LineString(WKBLineString::new(buf, byte_order.into(), 0, Xy))
            }
            WKBType::Polygon => {
                WKBGeometry::Polygon(WKBPolygon::new(buf, byte_order.into(), 0, Xy))
            }
            WKBType::MultiPoint => {
                WKBGeometry::MultiPoint(WKBMultiPoint::new(buf, byte_order.into(), Xy))
            }
            WKBType::MultiLineString => {
                WKBGeometry::MultiLineString(WKBMultiLineString::new(buf, byte_order.into(), Xy))
            }
            WKBType::MultiPolygon => {
                WKBGeometry::MultiPolygon(WKBMultiPolygon::new(buf, byte_order.into(), Xy))
            }
            WKBType::GeometryCollection => WKBGeometry::GeometryCollection(
                WKBGeometryCollection::try_new(buf, byte_order.into(), Xy)?,
            ),
            WKBType::PointZ => WKBGeometry::Point(WKBPoint::new(buf, byte_order.into(), 0, Xyz)),
            WKBType::LineStringZ => {
                WKBGeometry::LineString(WKBLineString::new(buf, byte_order.into(), 0, Xyz))
            }
            WKBType::PolygonZ => {
                WKBGeometry::Polygon(WKBPolygon::new(buf, byte_order.into(), 0, Xyz))
            }
            WKBType::MultiPointZ => {
                WKBGeometry::MultiPoint(WKBMultiPoint::new(buf, byte_order.into(), Xyz))
            }
            WKBType::MultiLineStringZ => {
                WKBGeometry::MultiLineString(WKBMultiLineString::new(buf, byte_order.into(), Xyz))
            }
            WKBType::MultiPolygonZ => {
                WKBGeometry::MultiPolygon(WKBMultiPolygon::new(buf, byte_order.into(), Xyz))
            }
            WKBType::GeometryCollectionZ => WKBGeometry::GeometryCollection(
                WKBGeometryCollection::try_new(buf, byte_order.into(), Xyz)?,
            ),
        };
        Ok(out)
    }

    pub fn into_point(self) -> WKBPoint<'a> {
        match self {
            WKBGeometry::Point(geom) => geom,
            _ => panic!(),
        }
    }

    pub fn into_line_string(self) -> WKBLineString<'a> {
        match self {
            WKBGeometry::LineString(geom) => geom,
            _ => panic!(),
        }
    }

    pub fn into_polygon(self) -> WKBPolygon<'a> {
        match self {
            WKBGeometry::Polygon(geom) => geom,
            _ => panic!(),
        }
    }

    pub fn into_multi_point(self) -> WKBMultiPoint<'a> {
        match self {
            WKBGeometry::MultiPoint(geom) => geom,
            _ => panic!(),
        }
    }

    pub fn into_multi_line_string(self) -> WKBMultiLineString<'a> {
        match self {
            WKBGeometry::MultiLineString(geom) => geom,
            _ => panic!(),
        }
    }

    pub fn into_multi_polygon(self) -> WKBMultiPolygon<'a> {
        match self {
            WKBGeometry::MultiPolygon(geom) => geom,
            _ => panic!(),
        }
    }

    pub fn dimension(&self) -> Dimensions {
        use WKBGeometry::*;
        match self {
            Point(g) => g.dimension(),
            LineString(g) => g.dimension(),
            Polygon(g) => g.dimension(),
            MultiPoint(g) => g.dimension(),
            MultiLineString(g) => g.dimension(),
            MultiPolygon(g) => g.dimension(),
            GeometryCollection(g) => g.dimension(),
        }
    }

    pub fn size(&self) -> u64 {
        use WKBGeometry::*;
        match self {
            Point(g) => g.size(),
            LineString(g) => g.size(),
            Polygon(g) => g.size(),
            MultiPoint(g) => g.size(),
            MultiLineString(g) => g.size(),
            MultiPolygon(g) => g.size(),
            GeometryCollection(g) => g.size(),
        }
    }
}

impl<'a> GeometryTrait for WKBGeometry<'a> {
    type T = f64;
    type PointType<'b> = WKBPoint<'a> where Self: 'b;
    type LineStringType<'b> = WKBLineString<'a> where Self: 'b;
    type PolygonType<'b> = WKBPolygon<'a> where Self: 'b;
    type MultiPointType<'b> = WKBMultiPoint<'a> where Self: 'b;
    type MultiLineStringType<'b> = WKBMultiLineString<'a> where Self: 'b;
    type MultiPolygonType<'b> = WKBMultiPolygon<'a> where Self: 'b;
    type GeometryCollectionType<'b> = WKBGeometryCollection<'a> where Self: 'b;
    type RectType<'b> = UnimplementedRect<f64> where Self: 'b;
    type TriangleType<'b> = UnimplementedTriangle<f64> where Self: 'b;
    type LineType<'b> = UnimplementedLine<f64> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dimension()
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        WKBPoint<'a>,
        WKBLineString<'a>,
        WKBPolygon<'a>,
        WKBMultiPoint<'a>,
        WKBMultiLineString<'a>,
        WKBMultiPolygon<'a>,
        WKBGeometryCollection<'a>,
        UnimplementedRect<f64>,
        UnimplementedTriangle<f64>,
        UnimplementedLine<f64>,
    > {
        use geo_traits::GeometryType as B;
        use WKBGeometry as A;
        match self {
            A::Point(p) => B::Point(p),
            A::LineString(ls) => B::LineString(ls),
            A::Polygon(ls) => B::Polygon(ls),
            A::MultiPoint(ls) => B::MultiPoint(ls),
            A::MultiLineString(ls) => B::MultiLineString(ls),
            A::MultiPolygon(ls) => B::MultiPolygon(ls),
            A::GeometryCollection(gc) => B::GeometryCollection(gc),
        }
    }
}

impl<'a> GeometryTrait for &'a WKBGeometry<'a> {
    type T = f64;
    type PointType<'b> = WKBPoint<'a> where Self: 'b;
    type LineStringType<'b> = WKBLineString<'a> where Self: 'b;
    type PolygonType<'b> = WKBPolygon<'a> where Self: 'b;
    type MultiPointType<'b> = WKBMultiPoint<'a> where Self: 'b;
    type MultiLineStringType<'b> = WKBMultiLineString<'a> where Self: 'b;
    type MultiPolygonType<'b> = WKBMultiPolygon<'a> where Self: 'b;
    type GeometryCollectionType<'b> = WKBGeometryCollection<'a> where Self: 'b;
    type RectType<'b> = UnimplementedRect<f64> where Self: 'b;
    type TriangleType<'b> = UnimplementedTriangle<f64> where Self: 'b;
    type LineType<'b> = UnimplementedLine<f64> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dimension()
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        WKBPoint<'a>,
        WKBLineString<'a>,
        WKBPolygon<'a>,
        WKBMultiPoint<'a>,
        WKBMultiLineString<'a>,
        WKBMultiPolygon<'a>,
        WKBGeometryCollection<'a>,
        UnimplementedRect<f64>,
        UnimplementedTriangle<f64>,
        UnimplementedLine<f64>,
    > {
        use geo_traits::GeometryType as B;
        use WKBGeometry as A;
        match self {
            A::Point(p) => B::Point(p),
            A::LineString(ls) => B::LineString(ls),
            A::Polygon(ls) => B::Polygon(ls),
            A::MultiPoint(ls) => B::MultiPoint(ls),
            A::MultiLineString(ls) => B::MultiLineString(ls),
            A::MultiPolygon(ls) => B::MultiPolygon(ls),
            A::GeometryCollection(gc) => B::GeometryCollection(gc),
        }
    }
}
