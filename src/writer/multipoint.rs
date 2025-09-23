use crate::common::WkbType;
use crate::error::WkbResult;
use crate::writer::point::{point_wkb_size, write_point};
use crate::writer::WriteOptions;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::MultiPointTrait;
use std::io::Write;

/// The number of bytes this MultiPoint will take up when encoded as WKB
pub fn multi_point_wkb_size(geom: &impl MultiPointTrait<T = f64>) -> usize {
    1 + 4 + 4 + (geom.num_points() * point_wkb_size(geom.dim()))
}

/// Write a MultiPoint geometry to a Writer encoded as WKB
pub fn write_multi_point(
    writer: &mut impl Write,
    geom: &impl MultiPointTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    // Byte order
    writer.write_u8(options.endianness.into())?;

    // Content
    match options.endianness {
        Endianness::LittleEndian => {
            write_multi_point_content::<LittleEndian>(writer, geom, options)
        }
        Endianness::BigEndian => write_multi_point_content::<BigEndian>(writer, geom, options),
    }
}

fn write_multi_point_content<B: ByteOrder>(
    writer: &mut impl Write,
    geom: &impl MultiPointTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    let wkb_type = WkbType::MultiPoint(geom.dim().try_into()?);
    writer.write_u32::<B>(wkb_type.into())?;

    // numPoints
    writer.write_u32::<B>(geom.num_points().try_into()?)?;

    for point in geom.points() {
        write_point(writer, &point, options)?;
    }

    Ok(())
}
