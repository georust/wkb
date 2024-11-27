use crate::common::WKBType;
use crate::error::WKBResult;
use crate::writer::coord::write_coord;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use core::f64;
use geo_traits::PointTrait;
use std::io::Write;

/// The byte length of a Point
pub fn point_wkb_size(dim: geo_traits::Dimensions) -> usize {
    let header = 1 + 4;
    let coords = dim.size() * 8;
    header + coords
}

/// Write a Point geometry to a Writer encoded as WKB
pub fn write_point(
    writer: &mut impl Write,
    geom: &impl PointTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    // Byte order header
    writer.write_u8(endianness.into())?;

    // Content
    match endianness {
        Endianness::LittleEndian => write_point_content::<LittleEndian>(writer, geom),
        Endianness::BigEndian => write_point_content::<BigEndian>(writer, geom),
    }
}

/// Write a Point geometry to a Writer encoded as WKB
fn write_point_content<B: ByteOrder>(
    writer: &mut impl Write,
    geom: &impl PointTrait<T = f64>,
) -> WKBResult<()> {
    let wkb_type = WKBType::Point(geom.dim().try_into()?);
    writer.write_u32::<B>(wkb_type.into())?;

    if let Some(coord) = geom.coord() {
        write_coord::<B>(writer, &coord)?;
    } else {
        // Write POINT EMPTY as f64::NAN values
        for _ in 0..geom.dim().size() {
            writer.write_f64::<B>(f64::NAN)?;
        }
    }

    Ok(())
}
