use geo_traits::to_geo::ToGeoGeometry;
use geo_traits::{CoordTrait, GeometryTrait, LineStringTrait};
use geo_types::Geometry;

use crate::reader::read_wkb;
use crate::writer::{
    write_geometry_collection, write_line_string, write_multi_line_string, write_multi_point,
    write_multi_polygon, write_point, write_polygon,
};
use crate::Endianness;

use super::data::*;

#[test]
fn round_trip_point() {
    let orig = point_2d();
    let mut buf = Vec::new();
    write_point(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Point(orig), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_point(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Point(orig), retour.to_geometry());
}

#[test]
fn round_trip_line_string() {
    let orig = linestring_2d();

    let mut buf = Vec::new();
    write_line_string(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::LineString(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_line_string(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::LineString(orig), retour.to_geometry());
}

#[test]
fn round_trip_polygon() {
    let orig = polygon_2d();

    let mut buf = Vec::new();
    write_polygon(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Polygon(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_polygon(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Polygon(orig), retour.to_geometry());
}

#[test]
fn round_trip_polygon_with_interior() {
    let orig = polygon_2d_with_interior();

    let mut buf = Vec::new();
    write_polygon(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Polygon(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_polygon(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Polygon(orig), retour.to_geometry());
}

#[test]
fn round_trip_multi_point() {
    let orig = multi_point_2d();

    let mut buf = Vec::new();
    write_multi_point(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiPoint(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_multi_point(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiPoint(orig), retour.to_geometry());
}

#[test]
fn round_trip_multi_line_string() {
    let orig = multi_line_string_2d();

    let mut buf = Vec::new();
    write_multi_line_string(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(
        Geometry::MultiLineString(orig.clone()),
        retour.to_geometry()
    );

    // Big endian
    let mut buf = Vec::new();
    write_multi_line_string(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiLineString(orig), retour.to_geometry());
}

#[test]
fn round_trip_multi_polygon() {
    let orig = multi_polygon_2d();

    let mut buf = Vec::new();
    write_multi_polygon(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiPolygon(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_multi_polygon(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiPolygon(orig), retour.to_geometry());
}

#[test]
fn round_trip_geometry_collection() {
    let orig = geometry_collection_2d();

    let mut buf = Vec::new();
    write_geometry_collection(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(
        Geometry::GeometryCollection(orig.clone()),
        retour.to_geometry()
    );

    // Big endian
    let mut buf = Vec::new();
    write_geometry_collection(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::GeometryCollection(orig), retour.to_geometry());
}

#[test]
fn wkb_geo_traits_lifetime() {
    // WKB representation of a LineString with 2 points at (1.0, 2.0) and (3.0, 4.0)
    let buf = vec![
        0x01, // little endian
        0x02, 0x00, 0x00, 0x00, // type: LineString (2)
        0x02, 0x00, 0x00, 0x00, // 2 points
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F, // x: 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // y: 2.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F, // x: 3.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // y: 4.0
    ];

    let coord;
    {
        let wkb = read_wkb(&buf).unwrap();
        let wkb_ref = &wkb;
        let wkb_ref_ref = &&wkb_ref;
        match wkb_ref_ref.as_type() {
            geo_traits::GeometryType::LineString(line_string) => {
                coord = line_string.coord(0);
            }
            _ => {
                panic!("Expected LineString");
            }
        }
    };

    assert!(coord.is_some());
    assert_eq!(coord.unwrap().x(), 1.0);
    assert_eq!(coord.unwrap().y(), 2.0);
}
