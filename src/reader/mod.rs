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

pub use geometry::Wkb;
use geometry_collection::GeometryCollection;
use linestring::LineString;
use multilinestring::MultiLineString;
use multipoint::MultiPoint;
use multipolygon::MultiPolygon;
use point::Point;
use polygon::Polygon;

use crate::error::WKBResult;

/// Parse a WKB byte slice into a geometry.
///
/// This is an alias for [`Wkb::try_new`].
pub fn read_wkb(buf: &[u8]) -> WKBResult<Wkb> {
    Wkb::try_new(buf)
}
