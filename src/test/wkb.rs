use geo_traits::to_geo::ToGeoGeometry;
use geo_traits::{CoordTrait, GeometryTrait, LineStringTrait, PointTrait, PolygonTrait};
use geo_types::Geometry;

use crate::reader::read_wkb;
use crate::writer::{
    write_geometry, write_geometry_collection, write_line_string, write_multi_line_string,
    write_multi_point, write_multi_polygon, write_point, write_polygon, WriteOptions,
};
use crate::Endianness;

use super::data::*;

#[test]
fn round_trip_point() {
    let orig = point_2d();
    let mut buf = Vec::new();
    write_point(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::LittleEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Point(orig), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_point(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::BigEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Point(orig), retour.to_geometry());
}

#[test]
fn round_trip_line_string() {
    let orig = linestring_2d();

    let mut buf = Vec::new();
    write_line_string(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::LittleEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::LineString(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_line_string(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::BigEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::LineString(orig), retour.to_geometry());
}

#[test]
fn round_trip_polygon() {
    let orig = polygon_2d();

    let mut buf = Vec::new();
    write_polygon(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::LittleEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Polygon(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_polygon(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::BigEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Polygon(orig), retour.to_geometry());
}

#[test]
fn round_trip_polygon_with_interior() {
    let orig = polygon_2d_with_interior();

    let mut buf = Vec::new();
    write_polygon(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::LittleEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Polygon(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_polygon(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::BigEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Polygon(orig), retour.to_geometry());
}

#[test]
fn round_trip_multi_point() {
    let orig = multi_point_2d();

    let mut buf = Vec::new();
    write_multi_point(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::LittleEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiPoint(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_multi_point(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::BigEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiPoint(orig), retour.to_geometry());
}

#[test]
fn round_trip_multi_line_string() {
    let orig = multi_line_string_2d();

    let mut buf = Vec::new();
    write_multi_line_string(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::LittleEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(
        Geometry::MultiLineString(orig.clone()),
        retour.to_geometry()
    );

    // Big endian
    let mut buf = Vec::new();
    write_multi_line_string(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::BigEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiLineString(orig), retour.to_geometry());
}

#[test]
fn round_trip_multi_polygon() {
    let orig = multi_polygon_2d();

    let mut buf = Vec::new();
    write_multi_polygon(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::LittleEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiPolygon(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_multi_polygon(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::BigEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiPolygon(orig), retour.to_geometry());
}

#[test]
fn round_trip_geometry_collection() {
    let orig = geometry_collection_2d();

    let mut buf = Vec::new();
    write_geometry_collection(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::LittleEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(
        Geometry::GeometryCollection(orig.clone()),
        retour.to_geometry()
    );

    // Big endian
    let mut buf = Vec::new();
    write_geometry_collection(
        &mut buf,
        &orig,
        &WriteOptions {
            endianness: Endianness::BigEndian,
        },
    )
    .unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::GeometryCollection(orig), retour.to_geometry());
}

#[test]
fn wkb_point_coord() {
    let p = point_2d();
    let mut buf = Vec::new();
    write_point(
        &mut buf,
        &p,
        &WriteOptions {
            endianness: Endianness::LittleEndian,
        },
    )
    .unwrap();
    let wkb = read_wkb(&buf).unwrap();
    let geo_traits::GeometryType::Point(point) = wkb.as_type() else {
        panic!("Expected Point");
    };
    let coord_slice = point.coord_slice();
    let x = f64::from_le_bytes(coord_slice[0..8].try_into().unwrap());
    let y = f64::from_le_bytes(coord_slice[8..16].try_into().unwrap());
    assert_eq!(x, 0.0);
    assert_eq!(y, 1.0);
    let coord = point.coord().unwrap();
    assert_eq!(coord.x(), x);
    assert_eq!(coord.y(), y);
}

#[test]
fn wkb_linestring_coords() {
    let ls = linestring_2d();
    let mut buf = Vec::new();
    write_line_string(
        &mut buf,
        &ls,
        &WriteOptions {
            endianness: Endianness::LittleEndian,
        },
    )
    .unwrap();
    let wkb = read_wkb(&buf).unwrap();
    let geo_traits::GeometryType::LineString(line_string) = wkb.as_type() else {
        panic!("Expected LineString");
    };
    let coord_slice = line_string.coords_slice();
    assert_eq!(coord_slice.len(), 32);
    let x0 = f64::from_le_bytes(coord_slice[0..8].try_into().unwrap());
    let y0 = f64::from_le_bytes(coord_slice[8..16].try_into().unwrap());
    let x1 = f64::from_le_bytes(coord_slice[16..24].try_into().unwrap());
    let y1 = f64::from_le_bytes(coord_slice[24..32].try_into().unwrap());
    assert_eq!(x0, 0.0);
    assert_eq!(y0, 1.0);
    assert_eq!(x1, 1.0);
    assert_eq!(y1, 2.0);
    let coord0 = line_string.coord(0).unwrap();
    let coord1 = line_string.coord(1).unwrap();
    assert_eq!(coord0.x(), x0);
    assert_eq!(coord0.y(), y0);
    assert_eq!(coord1.x(), x1);
    assert_eq!(coord1.y(), y1);
}

#[test]
fn wkb_polygon_coords() {
    let poly = polygon_2d();
    let mut buf = Vec::new();
    write_polygon(
        &mut buf,
        &poly,
        &WriteOptions {
            endianness: Endianness::LittleEndian,
        },
    )
    .unwrap();
    let wkb = read_wkb(&buf).unwrap();
    let geo_traits::GeometryType::Polygon(polygon) = wkb.as_type() else {
        panic!("Expected Polygon");
    };
    let exterior = polygon.exterior().unwrap();
    let coord_slice = exterior.coords_slice();
    assert_eq!(coord_slice.len(), 8 * 2 * exterior.num_coords());
    for (k, coord) in exterior.coords().enumerate() {
        let x = f64::from_le_bytes(coord_slice[16 * k..16 * k + 8].try_into().unwrap());
        let y = f64::from_le_bytes(coord_slice[16 * k + 8..16 * k + 16].try_into().unwrap());
        assert_eq!(x, coord.x());
        assert_eq!(y, coord.y());
        let coord = exterior.coord(k).unwrap();
        assert_eq!(x, coord.x());
        assert_eq!(y, coord.y());
    }
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

#[test]
fn wkb_geo_traits_specialized_lifetime() {
    let buf = vec![
        0x01, // little endian
        0x01, 0x00, 0x00, 0x00, // type: Point (1)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F, // x: 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // y: 2.0
    ];
    let coord = {
        let wkb = read_wkb(&buf).unwrap();
        let geo_traits::GeometryType::Point(point) = wkb.as_type() else {
            panic!("Expected Point");
        };

        let geo_traits::GeometryType::Point(point) = point.as_type() else {
            panic!("Expected Point");
        };

        point.coord()
    };

    assert!(coord.is_some());
    assert_eq!(coord.unwrap().x(), 1.0);
    assert_eq!(coord.unwrap().y(), 2.0);
}

fn test_wkb_buf_with_trailing_data(g: &Geometry) {
    let mut buf: Vec<u8> = Vec::new();
    write_geometry(
        &mut buf,
        g,
        &WriteOptions {
            endianness: Endianness::LittleEndian,
        },
    )
    .unwrap();

    let original_len = buf.len();
    buf.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);

    let wkb = read_wkb(&buf).unwrap();
    let trimmed_buf = wkb.buf();
    assert_eq!(trimmed_buf.len(), original_len);

    let wkb2 = read_wkb(trimmed_buf).unwrap();
    assert_eq!(*g, wkb2.to_geometry());
}

#[test]
fn wkb_point_buf_with_trailing_data() {
    test_wkb_buf_with_trailing_data(&Geometry::Point(point_2d()));
}

#[test]
fn wkb_linestring_buf_with_trailing_data() {
    test_wkb_buf_with_trailing_data(&Geometry::LineString(linestring_2d()));
}

#[test]
fn wkb_polygon_buf_with_trailing_data() {
    test_wkb_buf_with_trailing_data(&Geometry::Polygon(polygon_2d()));
}

#[test]
fn wkb_polygon_with_interior_buf_with_trailing_data() {
    test_wkb_buf_with_trailing_data(&Geometry::Polygon(polygon_2d_with_interior()));
}

#[test]
fn wkb_multi_point_buf_with_trailing_data() {
    test_wkb_buf_with_trailing_data(&Geometry::MultiPoint(multi_point_2d()));
}

#[test]
fn wkb_multi_line_string_buf_with_trailing_data() {
    test_wkb_buf_with_trailing_data(&Geometry::MultiLineString(multi_line_string_2d()));
}

#[test]
fn wkb_multi_polygon_buf_with_trailing_data() {
    test_wkb_buf_with_trailing_data(&Geometry::MultiPolygon(multi_polygon_2d()));
}

#[test]
fn wkb_geometry_collection_buf_with_trailing_data() {
    test_wkb_buf_with_trailing_data(&Geometry::GeometryCollection(geometry_collection_2d()));
}
