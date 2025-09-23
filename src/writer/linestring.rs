use crate::common::WkbType;
use crate::error::WkbResult;
use crate::writer::coord::write_coord;
use crate::writer::WriteOptions;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::LineStringTrait;
use std::io::Write;

/// The number of bytes this LineString will take up when encoded as WKB
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
    options: &WriteOptions,
) -> WkbResult<()> {
    // Byte order
    writer.write_u8(options.endianness.into())?;

    // Content
    match options.endianness {
        Endianness::LittleEndian => write_line_string_content::<LittleEndian>(writer, geom),
        Endianness::BigEndian => write_line_string_content::<BigEndian>(writer, geom),
    }
}

fn write_line_string_content<B: ByteOrder>(
    writer: &mut impl Write,
    geom: &impl LineStringTrait<T = f64>,
) -> WkbResult<()> {
    let wkb_type = WkbType::LineString(geom.dim().try_into()?);
    writer.write_u32::<B>(wkb_type.into())?;

    // numPoints
    writer.write_u32::<B>(geom.num_coords().try_into()?)?;

    for coord in geom.coords() {
        write_coord::<B>(writer, &coord)?;
    }

    Ok(())
}
