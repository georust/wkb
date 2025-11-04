//! Parse buffers containing WKB-encoded geometries.

// Each of the data structures in this module is intended to mirror the [WKB
// spec](https://portal.ogc.org/files/?artifact_id=25355).

mod coord;
mod geometry;
mod geometry_collection;
mod linearring;
mod linestring;
mod multilinestring;
mod multipoint;
mod multipolygon;
mod point;
mod polygon;
mod util;

pub use crate::common::Dimension;
pub use coord::Coord;
pub use geometry::Wkb;
pub use geometry_collection::GeometryCollection;
pub use linearring::LinearRing;
pub use linestring::LineString;
pub use multilinestring::MultiLineString;
pub use multipoint::MultiPoint;
pub use multipolygon::MultiPolygon;
pub use point::Point;
pub use polygon::Polygon;

use crate::error::WkbResult;

/// Parse a WKB byte slice into a geometry.
///
/// This is an alias for [`Wkb::try_new`].
pub fn read_wkb(buf: &[u8]) -> WkbResult<Wkb<'_>> {
    Wkb::try_new(buf)
}

/// The geometry type of the WKB object.
///
/// This is marked as non exhaustive because we do not currently support extended WKB types, such
/// as curves.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GeometryType {
    /// A WKB Point
    Point,
    /// A WKB LineString
    LineString,
    /// A WKB Polygon
    Polygon,
    /// A WKB MultiPoint
    MultiPoint,
    /// A WKB MultiLineString
    MultiLineString,
    /// A WKB MultiPolygon
    MultiPolygon,
    /// A WKB GeometryCollection
    GeometryCollection,
}

/// skip endianness and wkb type
const HEADER_BYTES: u64 = 5;
