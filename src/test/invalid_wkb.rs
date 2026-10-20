#[cfg(test)]
mod tests {
    use crate::common::Dimension::{Xy, Xyz};
    use crate::common::WkbType::{
        self, GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon, Point,
        Polygon,
    };
    use crate::error::WkbError;
    use crate::reader::{Wkb, MAX_WKB_NESTING_DEPTH};

    // --- Helper Functions ---
    fn make_wkb_header(typ: WkbType) -> Vec<u8> {
        let mut header = vec![0x01]; // little endian
        header.extend_from_slice(&u32::from(typ).to_le_bytes());
        header
    }

    // --- General WKB Errors ---
    #[test]
    fn test_wkb_invalid_byte_order() {
        let wkb_data = vec![0x02, 0x01, 0x00, 0x00, 0x00]; // Invalid byte order 0x02
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_buffer_too_short_for_header() {
        let wkb_data = vec![0x01, 0x01, 0x00]; // Only 3 bytes, header needs 5
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    // --- Unsupported Type Errors ---
    #[test]
    fn test_wkb_triangle_is_unsupported() {
        // TRIANGLE((0 0, 1 0, 1 1, 0 0)), as PostGIS writes it
        let mut wkb_data = vec![0x01];
        wkb_data.extend_from_slice(&17u32.to_le_bytes());
        wkb_data.extend_from_slice(&1u32.to_le_bytes());
        wkb_data.extend_from_slice(&4u32.to_le_bytes());
        for [x, y] in [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 0.0]] {
            wkb_data.extend_from_slice(&f64::to_le_bytes(x));
            wkb_data.extend_from_slice(&f64::to_le_bytes(y));
        }
        let err = Wkb::try_new(&wkb_data).unwrap_err();
        assert_eq!(
            err.to_string(),
            "General error: WKB type code out of range. Got: 17"
        );
    }

    #[test]
    fn test_wkb_triangle_z_is_unsupported() {
        // TRIANGLE Z((0 0 0, 1 0 0, 1 1 0, 0 0 0)), as PostGIS writes it
        let mut wkb_data = vec![0x01];
        // 1017 is the type code for a 3D triangle
        wkb_data.extend_from_slice(&1017u32.to_le_bytes());
        wkb_data.extend_from_slice(&1u32.to_le_bytes());
        wkb_data.extend_from_slice(&4u32.to_le_bytes());
        for [x, y, z] in [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 0.0, 0.0],
        ] {
            wkb_data.extend_from_slice(&f64::to_le_bytes(x));
            wkb_data.extend_from_slice(&f64::to_le_bytes(y));
            wkb_data.extend_from_slice(&f64::to_le_bytes(z));
        }
        let err = Wkb::try_new(&wkb_data).unwrap_err();
        assert_eq!(
            err.to_string(),
            "General error: WKB type code out of range. Got: 1017"
        );
    }

    // --- Point WKB Errors ---
    #[test]
    fn test_wkb_point_xy_buffer_too_short_for_coords() {
        let mut wkb_data = make_wkb_header(Point(Xy)); // 5 bytes
        wkb_data.extend_from_slice(&[0u8; 8]); // Only 8 bytes for coords, need 16
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_point_xyz_buffer_too_short_for_coords() {
        let mut wkb_data = make_wkb_header(Point(Xyz)); // 5 bytes
        wkb_data.extend_from_slice(&[0u8; 16]); // Only 16 bytes for coords, need 24
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    // --- LineString WKB Errors ---
    #[test]
    fn test_wkb_linestring_buffer_too_short_for_num_points() {
        // Missing the num_points field (4 bytes)
        let wkb_data = make_wkb_header(LineString(Xy));
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_linestring_num_points_too_large_for_buffer() {
        let mut wkb_data = make_wkb_header(LineString(Xy)); // 5 bytes
        wkb_data.extend_from_slice(&10u32.to_le_bytes()); // 10 points declared
        wkb_data.extend_from_slice(&1.0f64.to_le_bytes()); // Only 1 point's data (16 bytes)
        wkb_data.extend_from_slice(&2.0f64.to_le_bytes());
        // Total 5 + 4 + 16 = 25 bytes. Expected 5 + 4 + 10*16 = 169 bytes.
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_linestring_invalid_num_points_value() {
        let mut wkb_data = make_wkb_header(LineString(Xy));
        // u32::MAX num_points would cause massive size calculation if not for try_into error first
        wkb_data.extend_from_slice(&u32::MAX.to_le_bytes());
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    // --- Polygon WKB Errors ---
    #[test]
    fn test_wkb_polygon_buffer_too_short_for_num_rings() {
        // Missing the num_rings field (4 bytes)
        let wkb_data = make_wkb_header(Polygon(Xy));
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_polygon_buffer_too_short_for_ring_num_points() {
        let mut wkb_data = make_wkb_header(Polygon(Xy));
        // 1 ring, but the ring's num_points (4 bytes) is missing
        wkb_data.extend_from_slice(&1u32.to_le_bytes());
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_polygon_ring_num_points_too_large_for_buffer() {
        let mut wkb_data = make_wkb_header(Polygon(Xy));
        wkb_data.extend_from_slice(&1u32.to_le_bytes());
        // 1 ring declaring 4 points (closed), but only 1 point's worth of data follows
        wkb_data.extend_from_slice(&4u32.to_le_bytes());
        wkb_data.extend_from_slice(&0.0f64.to_le_bytes());
        wkb_data.extend_from_slice(&0.0f64.to_le_bytes());
        // Expected: 5 (poly header) + 4 (num_rings) + 4 (ring num_points) + 4*16 (coords) = 77
        // Actual:   5 + 4 + 4 + 16 = 29
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    // --- MultiPoint WKB Errors ---
    #[test]
    fn test_wkb_multipoint_buffer_too_short_for_num_multipoints_header() {
        // Missing num_points for the MultiPoint itself (4 bytes)
        let wkb_data = make_wkb_header(MultiPoint(Xy));
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_multipoint_buffer_too_short_for_contained_point_header() {
        let mut wkb_data = make_wkb_header(MultiPoint(Xy));
        // 1 point, but its WKB header (5 bytes: 1 byte order + 4 type) is missing
        wkb_data.extend_from_slice(&1u32.to_le_bytes());
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_multipoint_buffer_too_short_for_contained_point_coords() {
        let mut wkb_data = make_wkb_header(MultiPoint(Xy));
        wkb_data.extend_from_slice(&1u32.to_le_bytes()); // 1 point in multipoint
        wkb_data.extend(make_wkb_header(Point(Xy))); // Contained Point header (5 bytes)
        wkb_data.extend_from_slice(&[0u8; 8]); // Only 8 bytes for contained point's coords, needs 16
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    // --- MultiLineString WKB Errors ---
    #[test]
    fn test_wkb_multilinestring_buffer_too_short_for_num_multilinestrings_header() {
        // Missing num_linestrings for the MultiLineString itself (4 bytes)
        let wkb_data = make_wkb_header(MultiLineString(Xy));
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_multilinestring_buffer_too_short_for_contained_linestring_header() {
        let mut wkb_data = make_wkb_header(MultiLineString(Xy));
        // 1 linestring, but its WKB header (5 bytes) is missing
        wkb_data.extend_from_slice(&1u32.to_le_bytes());
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_multilinestring_buffer_too_short_for_contained_linestring_num_points() {
        let mut wkb_data = make_wkb_header(MultiLineString(Xy));
        wkb_data.extend_from_slice(&1u32.to_le_bytes());
        // The contained LineString has a header, but its num_points (4 bytes) is missing
        wkb_data.extend(make_wkb_header(LineString(Xy)));
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    // --- MultiPolygon WKB Errors ---
    #[test]
    fn test_wkb_multipolygon_buffer_too_short_for_num_multipolygons_header() {
        // Missing num_polygons for the MultiPolygon itself (4 bytes)
        let wkb_data = make_wkb_header(MultiPolygon(Xy));
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_multipolygon_buffer_too_short_for_contained_polygon_header() {
        let mut wkb_data = make_wkb_header(MultiPolygon(Xy));
        // 1 polygon, but its WKB header (5 bytes) is missing
        wkb_data.extend_from_slice(&1u32.to_le_bytes());
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    // --- GeometryCollection WKB Errors ---
    #[test]
    fn test_wkb_geomcollection_buffer_too_short_for_num_geometries_header() {
        // Missing num_geometries for the GeometryCollection itself (4 bytes)
        let wkb_data = make_wkb_header(GeometryCollection(Xy));
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_geomcollection_buffer_too_short_for_contained_geometry_header() {
        let mut wkb_data = make_wkb_header(GeometryCollection(Xy));
        // 1 geometry, but its WKB header (e.g. a Point, 5 bytes) is missing
        wkb_data.extend_from_slice(&1u32.to_le_bytes());
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    // --- Untrusted-input hardening (completes the "don't panic on invalid WKB" work in #74) ---

    fn make_nested_geometry_collections(levels: usize) -> Vec<u8> {
        // `levels` GeometryCollections each holding a single child, wrapping a terminal Point.
        let mut buf = Vec::new();
        for _ in 0..levels {
            buf.extend(make_wkb_header(GeometryCollection(Xy)));
            buf.extend_from_slice(&1u32.to_le_bytes()); // one geometry
        }
        buf.extend(make_wkb_header(Point(Xy)));
        buf.extend_from_slice(&0.0f64.to_le_bytes());
        buf.extend_from_slice(&0.0f64.to_le_bytes());
        buf
    }

    #[test]
    fn test_wkb_moderately_nested_geometry_collection_is_ok() {
        let wkb_data = make_nested_geometry_collections(50);
        assert!(Wkb::try_new(&wkb_data).is_ok());
    }

    #[test]
    fn test_wkb_nesting_at_depth_limit_is_ok() {
        let wkb_data = make_nested_geometry_collections(MAX_WKB_NESTING_DEPTH);
        assert!(Wkb::try_new(&wkb_data).is_ok());
    }

    #[test]
    fn test_wkb_nesting_beyond_depth_limit_errs() {
        let wkb_data = make_nested_geometry_collections(MAX_WKB_NESTING_DEPTH + 1);
        match Wkb::try_new(&wkb_data) {
            Err(WkbError::NestingDepthExceeded(limit)) => {
                assert_eq!(limit, MAX_WKB_NESTING_DEPTH)
            }
            other => panic!("expected NestingDepthExceeded, got {:?}", other),
        }
    }

    #[test]
    fn test_wkb_deeply_nested_geometry_collection_returns_err_not_stack_overflow() {
        // Without a depth limit this recurses until the stack overflows and aborts the process.
        let wkb_data = make_nested_geometry_collections(1_000);
        assert!(Wkb::try_new(&wkb_data).is_err());
    }

    // A header claiming u32::MAX elements must not pre-allocate gigabytes before the (missing)
    // body is read; it should just fail to read the first element.
    fn assert_huge_count_errs(typ: WkbType) {
        let mut wkb_data = make_wkb_header(typ);
        wkb_data.extend_from_slice(&u32::MAX.to_le_bytes());
        assert!(Wkb::try_new(&wkb_data).is_err());
    }

    #[test]
    fn test_wkb_multilinestring_huge_count_does_not_overallocate() {
        assert_huge_count_errs(MultiLineString(Xy));
    }

    #[test]
    fn test_wkb_polygon_huge_ring_count_does_not_overallocate() {
        assert_huge_count_errs(Polygon(Xy));
    }

    #[test]
    fn test_wkb_multipolygon_huge_count_does_not_overallocate() {
        assert_huge_count_errs(MultiPolygon(Xy));
    }

    #[test]
    fn test_wkb_geometry_collection_huge_count_does_not_overallocate() {
        assert_huge_count_errs(GeometryCollection(Xy));
    }
}
