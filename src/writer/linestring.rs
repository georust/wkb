use crate::common::WKBType;
use crate::error::WKBResult;
use crate::writer::coord::write_coord;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::LineStringTrait;
use std::io::Write;

/// The byte length of a LineString
pub fn line_string_wkb_size(geom: &impl LineStringTrait<T = f64>) -> usize {
    let header = 1 + 4 + 4;
    let each_coord = geom.dim().size() * 8;
    let all_coords = geom.num_coords() * each_coord;
    header + all_coords
}

/// Write a LineString geometry to a Writer encoded as WKB
pub fn write_line_string(
    writer: &mut impl Write,
    geom: &impl LineStringTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    // Byte order
    writer.write_u8(endianness.into()).unwrap();

    // Content
    match endianness {
        Endianness::LittleEndian => write_line_string_content::<LittleEndian>(writer, geom),
        Endianness::BigEndian => write_line_string_content::<BigEndian>(writer, geom),
    }
}

fn write_line_string_content<B: ByteOrder>(
    writer: &mut impl Write,
    geom: &impl LineStringTrait<T = f64>,
) -> WKBResult<()> {
    let wkb_type = WKBType::LineString(geom.dim().try_into()?);
    writer.write_u32::<LittleEndian>(wkb_type.into())?;

    // numPoints
    writer
        .write_u32::<B>(geom.num_coords().try_into().unwrap())
        .unwrap();

    for coord in geom.coords() {
        write_coord::<B>(writer, &coord)?;
    }

    Ok(())
}
