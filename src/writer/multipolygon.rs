use crate::common::WkbType;
use crate::error::WkbResult;
use crate::writer::polygon::{polygon_wkb_size, write_polygon};
use crate::writer::WriteOptions;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::MultiPolygonTrait;
use std::io::Write;

/// The number of bytes this MultiPolygon will take up when encoded as WKB
pub fn multi_polygon_wkb_size(geom: &impl MultiPolygonTrait<T = f64>) -> usize {
    let mut sum = 1 + 4 + 4;
    for polygon in geom.polygons() {
        sum += polygon_wkb_size(&polygon);
    }

    sum
}

/// Write a MultiPolygon geometry to a Writer encoded as WKB
pub fn write_multi_polygon(
    writer: &mut impl Write,
    geom: &impl MultiPolygonTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    // Byte order
    writer.write_u8(options.endianness.into())?;

    // Content
    match options.endianness {
        Endianness::LittleEndian => {
            write_multi_polygon_content::<LittleEndian>(writer, geom, options)
        }
        Endianness::BigEndian => write_multi_polygon_content::<BigEndian>(writer, geom, options),
    }
}

fn write_multi_polygon_content<B: ByteOrder>(
    writer: &mut impl Write,
    geom: &impl MultiPolygonTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    let wkb_type = WkbType::MultiPolygon(geom.dim().try_into()?);
    writer.write_u32::<B>(wkb_type.into())?;

    // numPolygons
    writer.write_u32::<B>(geom.num_polygons().try_into().unwrap())?;

    for polygon in geom.polygons() {
        write_polygon(writer, &polygon, options)?;
    }

    Ok(())
}
