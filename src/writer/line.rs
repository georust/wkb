use crate::common::WkbType;
use crate::error::WkbResult;
use crate::writer::coord::write_coord;
use crate::writer::WriteOptions;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::LineTrait;
use std::io::Write;

/// The number of bytes this Line will take up when encoded as WKB
pub fn line_wkb_size(geom: &impl LineTrait<T = f64>) -> usize {
    let header = 1 + 4 + 4;
    let each_coord = geom.dim().size() * 8;
    let all_coords = 2 * each_coord;
    header + all_coords
}

/// Write a Line geometry to a Writer encoded as WKB
pub fn write_line(
    writer: &mut impl Write,
    geom: &impl LineTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    // Byte order
    writer.write_u8(options.endianness.into()).unwrap();

    // Content
    match options.endianness {
        Endianness::LittleEndian => write_line_content::<LittleEndian>(writer, geom),
        Endianness::BigEndian => write_line_content::<BigEndian>(writer, geom),
    }
}

fn write_line_content<B: ByteOrder>(
    writer: &mut impl Write,
    geom: &impl LineTrait<T = f64>,
) -> WkbResult<()> {
    let wkb_type = WkbType::LineString(geom.dim().try_into()?);
    writer.write_u32::<B>(wkb_type.into())?;

    // numPoints
    writer.write_u32::<B>(2).unwrap();

    for coord in geom.coords() {
        write_coord::<B>(writer, &coord)?;
    }

    Ok(())
}
