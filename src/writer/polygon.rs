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
    let mut sum = 1 + 4 + 4;

    let each_coord = geom.dim().size() * 8;

    if let Some(ext_ring) = geom.exterior() {
        sum += 4 + (ext_ring.num_coords() * each_coord);
    }

    for int_ring in geom.interiors() {
        sum += 4 + (int_ring.num_coords() * each_coord);
    }

    sum
}

/// Write a Polygon geometry to a Writer encoded as WKB
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
    let num_rings = if geom.exterior().is_some() {
        1 + geom.num_interiors()
    } else {
        0
    };
    writer.write_u32::<B>(num_rings.try_into().unwrap())?;

    if let Some(ext_ring) = geom.exterior() {
        writer.write_u32::<B>(ext_ring.num_coords().try_into().unwrap())?;

        for coord in ext_ring.coords() {
            write_coord::<B>(writer, &coord)?;
        }
    }

    for int_ring in geom.interiors() {
        writer.write_u32::<B>(int_ring.num_coords().try_into().unwrap())?;

        for coord in int_ring.coords() {
            write_coord::<B>(writer, &coord)?;
        }
    }

    Ok(())
}
