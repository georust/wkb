#[cfg(test)]
mod tests {
    use crate::reader::Wkb;

    // --- Helper Functions ---
    fn make_wkb_header(type_id: u32, is_little_endian: bool) -> Vec<u8> {
        let mut header = vec![if is_little_endian { 0x01 } else { 0x00 }];
        if is_little_endian {
            header.extend_from_slice(&type_id.to_le_bytes());
        } else {
            header.extend_from_slice(&type_id.to_be_bytes());
        }
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

    // --- Point WKB Errors ---
    #[test]
    fn test_wkb_point_xy_buffer_too_short_for_coords() {
        let mut wkb_data = make_wkb_header(1, true); // Point XY, LE = 5 bytes
        wkb_data.extend_from_slice(&[0u8; 8]); // Only 8 bytes for coords, need 16
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_point_xyz_buffer_too_short_for_coords() {
        let mut wkb_data = make_wkb_header(1001, true); // Point XYZ, LE = 5 bytes
        wkb_data.extend_from_slice(&[0u8; 16]); // Only 16 bytes for coords, need 24
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    // --- LineString WKB Errors ---
    #[test]
    fn test_wkb_linestring_buffer_too_short_for_num_points() {
        let wkb_data = make_wkb_header(2, true); // LineString XY, LE = 5 bytes
                                                 // Missing num_points field (4 bytes)
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_linestring_num_points_too_large_for_buffer() {
        let mut wkb_data = make_wkb_header(2, true); // LineString XY, LE = 5 bytes
        wkb_data.extend_from_slice(&10u32.to_le_bytes()); // 10 points declared
        wkb_data.extend_from_slice(&1.0f64.to_le_bytes()); // Only 1 point's data (16 bytes)
        wkb_data.extend_from_slice(&2.0f64.to_le_bytes());
        // Total 5 + 4 + 16 = 25 bytes. Expected 5 + 4 + 10*16 = 169 bytes.
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_linestring_invalid_num_points_value() {
        let mut wkb_data = make_wkb_header(2, true); // LineString XY, LE
                                                     // u32::MAX num_points would cause massive size calculation if not for try_into error first
        wkb_data.extend_from_slice(&u32::MAX.to_le_bytes());
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    // --- Polygon WKB Errors ---
    #[test]
    fn test_wkb_polygon_buffer_too_short_for_num_rings() {
        let wkb_data = make_wkb_header(3, true); // Polygon XY, LE = 5 bytes
                                                 // Missing num_rings field (4 bytes)
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_polygon_buffer_too_short_for_ring_num_points() {
        let mut wkb_data = make_wkb_header(3, true); // Polygon XY, LE
        wkb_data.extend_from_slice(&1u32.to_le_bytes()); // 1 ring
                                                         // Missing num_points for the ring (4 bytes)
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_polygon_ring_num_points_too_large_for_buffer() {
        let mut wkb_data = make_wkb_header(3, true); // Polygon XY, LE
        wkb_data.extend_from_slice(&1u32.to_le_bytes()); // 1 ring
        wkb_data.extend_from_slice(&4u32.to_le_bytes()); // Ring has 4 points (closed)
                                                         // Provide data for only 1 point (16 bytes)
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
        let wkb_data = make_wkb_header(4, true); // MultiPoint XY, LE = 5 bytes
                                                 // Missing num_points for MultiPoint itself (4 bytes)
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_multipoint_buffer_too_short_for_contained_point_header() {
        let mut wkb_data = make_wkb_header(4, true); // MultiPoint XY, LE
        wkb_data.extend_from_slice(&1u32.to_le_bytes()); // 1 point in multipoint
                                                         // Missing the contained point's WKB header (5 bytes: 1 byte order + 4 type)
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_multipoint_buffer_too_short_for_contained_point_coords() {
        let mut wkb_data = make_wkb_header(4, true); // MultiPoint XY, LE
        wkb_data.extend_from_slice(&1u32.to_le_bytes()); // 1 point in multipoint
        wkb_data.extend(make_wkb_header(1, true)); // Contained Point XY header (5 bytes)
        wkb_data.extend_from_slice(&[0u8; 8]); // Only 8 bytes for contained point's coords, needs 16
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    // --- MultiLineString WKB Errors ---
    #[test]
    fn test_wkb_multilinestring_buffer_too_short_for_num_multilinestrings_header() {
        let wkb_data = make_wkb_header(5, true); // MultiLineString XY, LE = 5 bytes
                                                 // Missing num_linestrings for MultiLineString itself (4 bytes)
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_multilinestring_buffer_too_short_for_contained_linestring_header() {
        let mut wkb_data = make_wkb_header(5, true); // MultiLineString XY, LE
        wkb_data.extend_from_slice(&1u32.to_le_bytes()); // 1 linestring
                                                         // Missing contained linestring's WKB header (5 bytes)
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_multilinestring_buffer_too_short_for_contained_linestring_num_points() {
        let mut wkb_data = make_wkb_header(5, true); // MultiLineString XY, LE
        wkb_data.extend_from_slice(&1u32.to_le_bytes()); // 1 linestring
        wkb_data.extend(make_wkb_header(2, true)); // Contained LineString XY header (5 bytes)
                                                   // Missing contained linestring's num_points (4 bytes)
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    // --- MultiPolygon WKB Errors ---
    #[test]
    fn test_wkb_multipolygon_buffer_too_short_for_num_multipolygons_header() {
        let wkb_data = make_wkb_header(6, true); // MultiPolygon XY, LE = 5 bytes
                                                 // Missing num_polygons for MultiPolygon itself (4 bytes)
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_multipolygon_buffer_too_short_for_contained_polygon_header() {
        let mut wkb_data = make_wkb_header(6, true); // MultiPolygon XY, LE
        wkb_data.extend_from_slice(&1u32.to_le_bytes()); // 1 polygon
                                                         // Missing contained polygon's WKB header (5 bytes)
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    // --- GeometryCollection WKB Errors ---
    #[test]
    fn test_wkb_geomcollection_buffer_too_short_for_num_geometries_header() {
        let wkb_data = make_wkb_header(7, true); // GeometryCollection XY, LE = 5 bytes
                                                 // Missing num_geometries for GeometryCollection itself (4 bytes)
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_wkb_geomcollection_buffer_too_short_for_contained_geometry_header() {
        let mut wkb_data = make_wkb_header(7, true); // GeometryCollection XY, LE
        wkb_data.extend_from_slice(&1u32.to_le_bytes()); // 1 geometry
                                                         // Missing contained geometry's WKB header (e.g., a Point, 5 bytes)
        let result = Wkb::try_new(&wkb_data);
        assert!(result.is_err());
    }
}
