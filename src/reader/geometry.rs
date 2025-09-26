use std::io::Cursor;

use byteorder::ReadBytesExt;

use crate::common::{Dimension, WkbType};
use crate::error::{WkbError, WkbResult};
use crate::reader::{
    GeometryCollection, GeometryType, LineString, MultiLineString, MultiPoint, MultiPolygon, Point,
    Polygon,
};
use crate::Endianness;
use geo_traits::{
    Dimensions, GeometryTrait, UnimplementedLine, UnimplementedRect, UnimplementedTriangle,
};

/// Parse a WKB byte slice into a geometry.
///
/// An opaque object that implements [`GeometryTrait`]. Use methods provided by [`geo_traits`] to
/// access the underlying data.
///
/// The contained [dimension][geo_traits::Dimensions] will never be `Unknown`.
#[derive(Debug, Clone)]
pub struct Wkb<'a> {
    buf: &'a [u8],
    inner: WkbInner<'a>,
}

impl<'a> Wkb<'a> {
    /// Parse a WKB byte slice into a geometry.
    ///
    /// ### Performance
    ///
    /// WKB is not a zero-copy format because coordinates are not 8-byte aligned and because an
    /// initial scan needs to take place to know internal buffer offsets.
    ///
    /// This function does an initial pass over the WKB buffer to validate the contents and record
    /// the byte offsets for relevant coordinate slices but does not copy the underlying data to an
    /// alternate representation. This means that coordinates will **always be constant-time to
    /// access** but **not zero-copy**. This is because the raw WKB buffer is not 8-byte aligned,
    /// so when accessing a coordinate the underlying bytes need to be copied into a
    /// newly-allocated `f64`.
    pub fn try_new(buf: &'a [u8]) -> WkbResult<Self> {
        let inner = WkbInner::try_new(buf)?;
        Ok(Self { buf, inner })
    }

    /// Return the [Dimension] of this geometry.
    pub fn dimension(&self) -> Dimension {
        use WkbInner::*;
        match &self.inner {
            Point(g) => g.dimension(),
            LineString(g) => g.dimension(),
            Polygon(g) => g.dimension(),
            MultiPoint(g) => g.dimension(),
            MultiLineString(g) => g.dimension(),
            MultiPolygon(g) => g.dimension(),
            GeometryCollection(g) => g.dimension(),
        }
    }

    /// Return the [GeometryType] of this geometry.
    pub fn geometry_type(&self) -> GeometryType {
        use WkbInner::*;
        match &self.inner {
            Point(_) => GeometryType::Point,
            LineString(_) => GeometryType::LineString,
            Polygon(_) => GeometryType::Polygon,
            MultiPoint(_) => GeometryType::MultiPoint,
            MultiLineString(_) => GeometryType::MultiLineString,
            MultiPolygon(_) => GeometryType::MultiPolygon,
            GeometryCollection(_) => GeometryType::GeometryCollection,
        }
    }

    /// Return the underlying buffer of this WKB geometry.
    #[inline]
    pub fn buf(&self) -> &'a [u8] {
        self.buf
    }

    pub(crate) fn size(&self) -> u64 {
        use WkbInner::*;
        match &self.inner {
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

/// This is **not** exported publicly because we don't want to expose the enum variants publicly.
#[derive(Debug, Clone)]
pub(crate) enum WkbInner<'a> {
    Point(Point<'a>),
    LineString(LineString<'a>),
    Polygon(Polygon<'a>),
    MultiPoint(MultiPoint<'a>),
    MultiLineString(MultiLineString<'a>),
    MultiPolygon(MultiPolygon<'a>),
    GeometryCollection(GeometryCollection<'a>),
}

impl<'a> WkbInner<'a> {
    fn try_new(buf: &'a [u8]) -> WkbResult<Self> {
        let mut reader = Cursor::new(buf);
        let byte_order = Endianness::try_from(reader.read_u8()?)
            .map_err(|_| WkbError::General("Invalid byte order".to_string()))?;
        let wkb_type = WkbType::from_buffer(buf)?;

        let out = match wkb_type {
            WkbType::Point(dim) => Self::Point(Point::try_new(buf, byte_order, 0, dim)?),
            WkbType::LineString(dim) => {
                Self::LineString(LineString::try_new(buf, byte_order, 0, dim)?)
            }
            WkbType::Polygon(dim) => Self::Polygon(Polygon::try_new(buf, byte_order, 0, dim)?),
            WkbType::MultiPoint(dim) => {
                Self::MultiPoint(MultiPoint::try_new(buf, byte_order, dim)?)
            }
            WkbType::MultiLineString(dim) => {
                Self::MultiLineString(MultiLineString::try_new(buf, byte_order, dim)?)
            }
            WkbType::MultiPolygon(dim) => {
                Self::MultiPolygon(MultiPolygon::try_new(buf, byte_order, dim)?)
            }
            WkbType::GeometryCollection(dim) => {
                Self::GeometryCollection(GeometryCollection::try_new(buf, byte_order, dim)?)
            }
        };
        Ok(out)
    }
}

impl<'a> GeometryTrait for Wkb<'a> {
    type T = f64;
    type PointType<'b>
        = Point<'a>
    where
        Self: 'b;
    type LineStringType<'b>
        = LineString<'a>
    where
        Self: 'b;
    type PolygonType<'b>
        = Polygon<'a>
    where
        Self: 'b;
    type MultiPointType<'b>
        = MultiPoint<'a>
    where
        Self: 'b;
    type MultiLineStringType<'b>
        = MultiLineString<'a>
    where
        Self: 'b;
    type MultiPolygonType<'b>
        = MultiPolygon<'a>
    where
        Self: 'b;
    type GeometryCollectionType<'b>
        = GeometryCollection<'a>
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
        use geo_traits::GeometryType as B;
        use WkbInner as A;
        match &self.inner {
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

impl<'a> GeometryTrait for &Wkb<'a> {
    type T = f64;
    type PointType<'b>
        = Point<'a>
    where
        Self: 'b;
    type LineStringType<'b>
        = LineString<'a>
    where
        Self: 'b;
    type PolygonType<'b>
        = Polygon<'a>
    where
        Self: 'b;
    type MultiPointType<'b>
        = MultiPoint<'a>
    where
        Self: 'b;
    type MultiLineStringType<'b>
        = MultiLineString<'a>
    where
        Self: 'b;
    type MultiPolygonType<'b>
        = MultiPolygon<'a>
    where
        Self: 'b;
    type GeometryCollectionType<'b>
        = GeometryCollection<'a>
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
        use geo_traits::GeometryType as B;
        use WkbInner as A;
        match &self.inner {
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

// Specialized implementations on each WKT concrete type.

macro_rules! impl_specialization {
    ($geometry_type:ident) => {
        impl<'a> GeometryTrait for $geometry_type<'a> {
            type T = f64;
            type PointType<'b>
                = Point<'a>
            where
                Self: 'b;
            type LineStringType<'b>
                = LineString<'a>
            where
                Self: 'b;
            type PolygonType<'b>
                = Polygon<'a>
            where
                Self: 'b;
            type MultiPointType<'b>
                = MultiPoint<'a>
            where
                Self: 'b;
            type MultiLineStringType<'b>
                = MultiLineString<'a>
            where
                Self: 'b;
            type MultiPolygonType<'b>
                = MultiPolygon<'a>
            where
                Self: 'b;
            type GeometryCollectionType<'b>
                = GeometryCollection<'a>
            where
                Self: 'b;
            type RectType<'b>
                = geo_traits::UnimplementedRect<f64>
            where
                Self: 'b;
            type LineType<'b>
                = geo_traits::UnimplementedLine<f64>
            where
                Self: 'b;
            type TriangleType<'b>
                = geo_traits::UnimplementedTriangle<f64>
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
                geo_traits::GeometryType::$geometry_type(self)
            }
        }

        impl<'a> GeometryTrait for &$geometry_type<'a> {
            type T = f64;
            type PointType<'b>
                = Point<'a>
            where
                Self: 'b;
            type LineStringType<'b>
                = LineString<'a>
            where
                Self: 'b;
            type PolygonType<'b>
                = Polygon<'a>
            where
                Self: 'b;
            type MultiPointType<'b>
                = MultiPoint<'a>
            where
                Self: 'b;
            type MultiLineStringType<'b>
                = MultiLineString<'a>
            where
                Self: 'b;
            type MultiPolygonType<'b>
                = MultiPolygon<'a>
            where
                Self: 'b;
            type GeometryCollectionType<'b>
                = GeometryCollection<'a>
            where
                Self: 'b;
            type RectType<'b>
                = geo_traits::UnimplementedRect<f64>
            where
                Self: 'b;
            type LineType<'b>
                = geo_traits::UnimplementedLine<f64>
            where
                Self: 'b;
            type TriangleType<'b>
                = geo_traits::UnimplementedTriangle<f64>
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
                geo_traits::GeometryType::$geometry_type(self)
            }
        }
    };
}

impl_specialization!(Point);
impl_specialization!(LineString);
impl_specialization!(Polygon);
impl_specialization!(MultiPoint);
impl_specialization!(MultiLineString);
impl_specialization!(MultiPolygon);
impl_specialization!(GeometryCollection);
