#[cfg(test)]
mod tests {
    use crate::reader::Wkb;

    const Z_FLAG: u32 = 0x80000000;
    const M_FLAG: u32 = 0x40000000;
    const SRID_FLAG: u32 = 0x20000000;

    // --- Helper Functions ---
    fn make_ewkb_header(
        type_id_base: u32,
        has_z: bool,
        has_m: bool,
        has_srid: bool,
        is_little_endian: bool,
    ) -> Vec<u8> {
        let mut type_val = type_id_base;
        if has_z {
            type_val |= Z_FLAG;
        }
        if has_m {
            type_val |= M_FLAG;
        }
        if has_srid {
            type_val |= SRID_FLAG;
        }

        let mut header = vec![if is_little_endian { 0x01 } else { 0x00 }];
        if is_little_endian {
            header.extend_from_slice(&type_val.to_le_bytes());
        } else {
            header.extend_from_slice(&type_val.to_be_bytes());
        }
        header
    }

    fn srid_bytes(srid: u32, is_little_endian: bool) -> [u8; 4] {
        if is_little_endian {
            srid.to_le_bytes()
        } else {
            srid.to_be_bytes()
        }
    }

    // --- EWKB Point Errors ---
    #[test]
    fn test_ewkb_point_srid_flag_but_no_srid_data() {
        let mut ewkb_data = make_ewkb_header(1, false, false, true, true); // Point XY, with SRID flag, LE
                                                                           // Missing SRID data (4 bytes), then coords should start
        ewkb_data.extend_from_slice(&1.0f64.to_le_bytes()); // X
        ewkb_data.extend_from_slice(&2.0f64.to_le_bytes()); // Y
        let result = Wkb::try_new(&ewkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_ewkb_point_srid_flag_buffer_too_short_for_srid_value() {
        let mut ewkb_data = make_ewkb_header(1, false, false, true, true); // Point XY, with SRID flag, LE
        ewkb_data.extend_from_slice(&[0u8; 2]); // Only 2 bytes for SRID, needs 4
        let result = Wkb::try_new(&ewkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_ewkb_point_with_srid_buffer_too_short_for_coords() {
        let mut ewkb_data = make_ewkb_header(1, false, false, true, true); // Point XY, with SRID flag, LE
        ewkb_data.extend_from_slice(&srid_bytes(4326, true)); // SRID (4 bytes)
        ewkb_data.extend_from_slice(&[0u8; 8]); // Only 8 bytes for coords, need 16
        let result = Wkb::try_new(&ewkb_data);
        assert!(result.is_err());
    }

    // --- EWKB LineString Errors ---
    #[test]
    fn test_ewkb_linestring_with_srid_buffer_too_short_for_num_points() {
        let mut ewkb_data = make_ewkb_header(2, false, false, true, true); // LineString XY, SRID, LE
        ewkb_data.extend_from_slice(&srid_bytes(4326, true)); // SRID (4 bytes)
                                                              // num_points field (4 bytes) is missing
        let result = Wkb::try_new(&ewkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_ewkb_linestring_with_srid_num_points_too_large_for_buffer() {
        let mut ewkb_data = make_ewkb_header(2, false, false, true, true); // LineString XY, SRID, LE
        ewkb_data.extend_from_slice(&srid_bytes(4326, true)); // SRID (4 bytes)
        ewkb_data.extend_from_slice(&10u32.to_le_bytes()); // 10 points declared
        ewkb_data.extend_from_slice(&1.0f64.to_le_bytes()); // Only 1 point's data (16 bytes)
        ewkb_data.extend_from_slice(&2.0f64.to_le_bytes());
        let result = Wkb::try_new(&ewkb_data);
        assert!(result.is_err());
    }

    // --- EWKB Polygon Errors ---
    #[test]
    fn test_ewkb_polygon_with_srid_buffer_too_short_for_num_rings() {
        let mut ewkb_data = make_ewkb_header(3, false, false, true, true); // Polygon XY, SRID, LE
        ewkb_data.extend_from_slice(&srid_bytes(4326, true)); // SRID (4 bytes)
                                                              // num_rings field (4 bytes) is missing
        let result = Wkb::try_new(&ewkb_data);
        assert!(result.is_err());
    }

    // --- EWKB MultiPoint Errors ---
    #[test]
    fn test_ewkb_multipoint_srid_flag_but_no_srid_data_for_outer_geom() {
        let mut ewkb_data = make_ewkb_header(4, false, false, true, true); // MultiPoint XY, SRID, LE
                                                                           // Outer SRID missing
        ewkb_data.extend_from_slice(&1u32.to_le_bytes()); // 1 point
                                                          // Contained point (must also have its own header, potentially SRID)
        let mut contained_point = make_ewkb_header(1, false, false, false, true); // Point XY, no SRID, LE
        contained_point.extend_from_slice(&1.0f64.to_le_bytes());
        contained_point.extend_from_slice(&2.0f64.to_le_bytes());
        ewkb_data.extend(contained_point);
        let result = Wkb::try_new(&ewkb_data);
        assert!(result.is_err());
    }

    // --- Z/M Dimension Mismatch (EWKB specific example) ---
    #[test]
    fn test_ewkb_point_xyz_flag_but_xy_data() {
        let mut ewkb_data = make_ewkb_header(1, true, false, false, true); // Point XYZ (Z_FLAG), no SRID, LE
        ewkb_data.extend_from_slice(&1.0f64.to_le_bytes()); // X
        ewkb_data.extend_from_slice(&2.0f64.to_le_bytes()); // Y
                                                            // Missing Z coordinate (8 bytes)
        let result = Wkb::try_new(&ewkb_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_ewkb_linestring_xyzm_flag_but_xyz_data_for_points() {
        let type_id_base = 2; // LineString
        let mut ewkb_data = make_ewkb_header(type_id_base, true, true, false, true); // LineString XYZM, no SRID, LE
        ewkb_data.extend_from_slice(&1u32.to_le_bytes()); // 1 point
                                                          // Coords for 1 point (XYZM = 32 bytes expected per point)
        ewkb_data.extend_from_slice(&1.0f64.to_le_bytes()); // X
        ewkb_data.extend_from_slice(&2.0f64.to_le_bytes()); // Y
        ewkb_data.extend_from_slice(&3.0f64.to_le_bytes()); // Z
                                                            // Missing M coordinate (8 bytes)
        let result = Wkb::try_new(&ewkb_data);
        assert!(result.is_err());
    }
}
