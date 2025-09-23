use crate::common::WkbType;
use crate::error::WkbResult;
use crate::writer::coord::write_coord;
use crate::writer::WriteOptions;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::TriangleTrait;
use std::io::Write;

/// The number of bytes this Triangle will take up when encoded as WKB
pub fn triangle_wkb_size(geom: &impl TriangleTrait<T = f64>) -> usize {
    let header = 1 + 4 + 4;
    let each_coord = geom.dim().size() * 8;
    let all_coords = 4 * each_coord;
    header + all_coords
}

/// Write a Triangle geometry to a Writer encoded as WKB
pub fn write_triangle(
    writer: &mut impl Write,
    geom: &impl TriangleTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    // Byte order
    writer.write_u8(options.endianness.into())?;

    // Content
    match options.endianness {
        Endianness::LittleEndian => write_triangle_content::<LittleEndian>(writer, geom),
        Endianness::BigEndian => write_triangle_content::<BigEndian>(writer, geom),
    }
}

fn write_triangle_content<B: ByteOrder>(
    writer: &mut impl Write,
    geom: &impl TriangleTrait<T = f64>,
) -> WkbResult<()> {
    let wkb_type = WkbType::Polygon(geom.dim().try_into()?);
    writer.write_u32::<B>(wkb_type.into())?;

    // numRings
    let num_rings = 1;
    writer.write_u32::<B>(num_rings)?;

    let num_coords = 4;
    writer.write_u32::<B>(num_coords)?;

    write_coord::<B>(writer, &geom.first())?;
    write_coord::<B>(writer, &geom.second())?;
    write_coord::<B>(writer, &geom.third())?;
    write_coord::<B>(writer, &geom.first())?;

    Ok(())
}
