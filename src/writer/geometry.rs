use crate::error::WkbResult;
use crate::writer::{
    geometry_collection_wkb_size, line_string_wkb_size, line_wkb_size, multi_line_string_wkb_size,
    multi_point_wkb_size, multi_polygon_wkb_size, point_wkb_size, polygon_wkb_size, rect_wkb_size,
    triangle_wkb_size, write_geometry_collection, write_line, write_line_string,
    write_multi_line_string, write_multi_point, write_multi_polygon, write_point, write_polygon,
    write_rect, write_triangle, WriteOptions,
};
use geo_traits::{GeometryTrait, GeometryType};
use std::io::Write;

/// The number of bytes this geometry will take up when encoded as WKB
pub fn geometry_wkb_size(geom: &impl GeometryTrait<T = f64>) -> usize {
    use GeometryType::*;
    match geom.as_type() {
        Point(_) => point_wkb_size(geom.dim()),
        LineString(ls) => line_string_wkb_size(ls),
        Polygon(p) => polygon_wkb_size(p),
        MultiPoint(mp) => multi_point_wkb_size(mp),
        MultiLineString(ml) => multi_line_string_wkb_size(ml),
        MultiPolygon(mp) => multi_polygon_wkb_size(mp),
        GeometryCollection(gc) => geometry_collection_wkb_size(gc),
        Rect(r) => rect_wkb_size(r),
        Triangle(tri) => triangle_wkb_size(tri),
        Line(line) => line_wkb_size(line),
    }
}

/// Write a Geometry to a Writer encoded as WKB
pub fn write_geometry(
    writer: &mut impl Write,
    geom: &impl GeometryTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    use GeometryType::*;
    match geom.as_type() {
        Point(p) => write_point(writer, p, options),
        LineString(ls) => write_line_string(writer, ls, options),
        Polygon(p) => write_polygon(writer, p, options),
        MultiPoint(mp) => write_multi_point(writer, mp, options),
        MultiLineString(ml) => write_multi_line_string(writer, ml, options),
        MultiPolygon(mp) => write_multi_polygon(writer, mp, options),
        GeometryCollection(gc) => write_geometry_collection(writer, gc, options),
        Rect(r) => write_rect(writer, r, options),
        Triangle(tri) => write_triangle(writer, tri, options),
        Line(line) => write_line(writer, line, options),
    }
}
