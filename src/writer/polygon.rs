use crate::common::WkbType;
use crate::error::WkbResult;
use crate::writer::coord::write_coord;
use crate::writer::WriteOptions;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::{LineStringTrait, PolygonTrait};
use std::io::Write;

/// The number of bytes this Polygon will take up when encoded as WKB
pub fn polygon_wkb_size(geom: &impl PolygonTrait<T = f64>) -> usize {
    let each_coord = geom.dim().size() * 8;
    let num_coords = geom.exterior().map_or(0, |ring| ring.num_coords())
        + geom
            .interiors()
            .map(|ring| ring.num_coords())
            .sum::<usize>();

    1 + 4 + 4 + (4 * wkb_num_rings(geom)) + (num_coords * each_coord)
}

/// The number of rings this Polygon will have when encoded as WKB.
///
/// If the Polygon has interior rings but no exterior ring, the WKB contains an
/// empty exterior ring, because the first ring is always the exterior.
fn wkb_num_rings(geom: &impl PolygonTrait<T = f64>) -> usize {
    match (geom.exterior(), geom.num_interiors()) {
        (None, 0) => 0,
        (_, num_interiors) => 1 + num_interiors,
    }
}

/// Write a Polygon geometry to a Writer encoded as WKB
///
/// If the Polygon has interior rings but no exterior ring, this function writes an empty
/// exterior ring (zero points) before the interior rings. GDAL can write and read this
/// encoding. GEOS and JTS do not accept a Polygon with an empty exterior ring and non-empty
/// interior rings, and return an error when they read it. PostGIS returns an error when its
/// default parser checks are enabled.
pub fn write_polygon(
    writer: &mut impl Write,
    geom: &impl PolygonTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    // Byte order
    writer.write_u8(options.endianness.into())?;

    // Content
    match options.endianness {
        Endianness::LittleEndian => write_polygon_content::<LittleEndian>(writer, geom),
        Endianness::BigEndian => write_polygon_content::<BigEndian>(writer, geom),
    }
}

fn write_polygon_content<B: ByteOrder>(
    writer: &mut impl Write,
    geom: &impl PolygonTrait<T = f64>,
) -> WkbResult<()> {
    let wkb_type = WkbType::Polygon(geom.dim().try_into()?);
    writer.write_u32::<B>(wkb_type.into())?;

    // numRings
    let num_rings = wkb_num_rings(geom);
    writer.write_u32::<B>(num_rings.try_into()?)?;

    if num_rings == 0 {
        return Ok(());
    }

    match geom.exterior() {
        Some(ext_ring) => write_ring::<B>(writer, &ext_ring)?,
        // Empty exterior ring
        None => writer.write_u32::<B>(0)?,
    }

    for int_ring in geom.interiors() {
        write_ring::<B>(writer, &int_ring)?;
    }

    Ok(())
}

fn write_ring<B: ByteOrder>(
    writer: &mut impl Write,
    ring: &impl LineStringTrait<T = f64>,
) -> WkbResult<()> {
    // numPoints
    writer.write_u32::<B>(ring.num_coords().try_into()?)?;

    for coord in ring.coords() {
        write_coord::<B>(writer, &coord)?;
    }

    Ok(())
}
