use geo_traits::{
    CoordTrait, Dimensions, GeometryTrait, GeometryType, RectTrait,
    UnimplementedGeometryCollection, UnimplementedLine, UnimplementedLineString,
    UnimplementedMultiLineString, UnimplementedMultiPoint, UnimplementedMultiPolygon,
    UnimplementedPoint, UnimplementedPolygon, UnimplementedTriangle,
};
use geo_types::{
    coord, line_string, point, polygon, Geometry, GeometryCollection, LineString, MultiLineString,
    MultiPoint, MultiPolygon, Point, Polygon, Rect,
};

pub(super) fn point_2d() -> Point {
    point!(
        x: 0., y: 1.
    )
}

pub(super) fn linestring_2d() -> LineString {
    line_string![
        (x: 0., y: 1.),
        (x: 1., y: 2.)
    ]
}

pub(super) fn polygon_2d() -> Polygon {
    polygon![
        (x: -111., y: 45.),
        (x: -111., y: 41.),
        (x: -104., y: 41.),
        (x: -104., y: 45.),
    ]
}

pub(super) fn polygon_2d_with_interior() -> Polygon {
    polygon!(
        exterior: [
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        ],
        interiors: [
            [
                (x: -110., y: 44.),
                (x: -110., y: 42.),
                (x: -105., y: 42.),
                (x: -105., y: 44.),
            ],
        ],
    )
}

pub(super) fn polygon_2d_empty() -> Polygon {
    Polygon::new(LineString::new(vec![]), vec![])
}

/// A polygon with an empty exterior ring and one interior ring.
pub(super) fn polygon_2d_empty_exterior_with_interior() -> Polygon {
    Polygon::new(
        LineString::new(vec![]),
        vec![line_string![
            (x: 1., y: 2.),
            (x: 3., y: 4.),
            (x: 5., y: 2.),
            (x: 1., y: 2.)
        ]],
    )
}

pub(super) fn rect_2d() -> Rect {
    Rect::new(coord! { x: 0., y: 1. }, coord! { x: 2., y: 3. })
}

pub(super) fn multi_point_2d() -> MultiPoint {
    MultiPoint::new(vec![
        point!(
            x: 0., y: 1.
        ),
        point!(
            x: 1., y: 2.
        ),
    ])
}

pub(super) fn multi_line_string_2d() -> MultiLineString {
    MultiLineString::new(vec![
        line_string![
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        ],
        line_string![
            (x: -110., y: 44.),
            (x: -110., y: 42.),
            (x: -105., y: 42.),
            (x: -105., y: 44.),
        ],
    ])
}

pub(super) fn multi_polygon_2d() -> MultiPolygon {
    MultiPolygon::new(vec![
        polygon![
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        ],
        polygon!(
            exterior: [
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ],
            interiors: [
                [
                    (x: -110., y: 44.),
                    (x: -110., y: 42.),
                    (x: -105., y: 42.),
                    (x: -105., y: 44.),
                ],
            ],
        ),
    ])
}

pub(super) fn geometry_collection_2d() -> GeometryCollection {
    GeometryCollection::new_from(vec![
        Geometry::Point(point_2d()),
        Geometry::LineString(linestring_2d()),
        Geometry::Polygon(polygon_2d()),
        Geometry::Polygon(polygon_2d_with_interior()),
        Geometry::MultiPoint(multi_point_2d()),
        Geometry::MultiLineString(multi_line_string_2d()),
        Geometry::MultiPolygon(multi_polygon_2d()),
    ])
}

/// A 3D coordinate. geo-types only provides 2D coordinates.
#[derive(Clone, Copy)]
pub(super) struct Coord3d {
    x: f64,
    y: f64,
    z: f64,
}

impl CoordTrait for Coord3d {
    type T = f64;

    fn dim(&self) -> Dimensions {
        Dimensions::Xyz
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn nth_or_panic(&self, n: usize) -> f64 {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Coord3d has no dimension {n}"),
        }
    }
}

/// A 3D Rect. geo-types only provides a 2D Rect.
pub(super) struct Rect3d {
    min: Coord3d,
    max: Coord3d,
}

impl RectTrait for Rect3d {
    type CoordType<'a> = Coord3d;

    fn min(&self) -> Self::CoordType<'_> {
        self.min
    }

    fn max(&self) -> Self::CoordType<'_> {
        self.max
    }
}

impl GeometryTrait for Rect3d {
    type T = f64;
    type PointType<'a> = UnimplementedPoint<f64>;
    type LineStringType<'a> = UnimplementedLineString<f64>;
    type PolygonType<'a> = UnimplementedPolygon<f64>;
    type MultiPointType<'a> = UnimplementedMultiPoint<f64>;
    type MultiLineStringType<'a> = UnimplementedMultiLineString<f64>;
    type MultiPolygonType<'a> = UnimplementedMultiPolygon<f64>;
    type GeometryCollectionType<'a> = UnimplementedGeometryCollection<f64>;
    type RectType<'a> = Rect3d;
    type TriangleType<'a> = UnimplementedTriangle<f64>;
    type LineType<'a> = UnimplementedLine<f64>;

    fn dim(&self) -> Dimensions {
        Dimensions::Xyz
    }

    fn as_type(
        &self,
    ) -> GeometryType<
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
        GeometryType::Rect(self)
    }
}

pub(super) fn rect_3d() -> Rect3d {
    Rect3d {
        min: Coord3d {
            x: 0.,
            y: 1.,
            z: 10.,
        },
        max: Coord3d {
            x: 2.,
            y: 3.,
            z: 20.,
        },
    }
}
