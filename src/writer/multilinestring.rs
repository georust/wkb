use crate::common::WkbType;
use crate::error::WkbResult;
use crate::writer::linestring::{line_string_wkb_size, write_line_string};
use crate::writer::WriteOptions;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::MultiLineStringTrait;
use std::io::Write;

/// The number of bytes this MultiLineString will take up when encoded as WKB
pub fn multi_line_string_wkb_size(geom: &impl MultiLineStringTrait<T = f64>) -> usize {
    let mut sum = 1 + 4 + 4;
    for line_string in geom.line_strings() {
        sum += line_string_wkb_size(&line_string);
    }

    sum
}

/// Write a MultiLineString geometry to a Writer encoded as WKB
pub fn write_multi_line_string(
    writer: &mut impl Write,
    geom: &impl MultiLineStringTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    // Byte order
    writer.write_u8(options.endianness.into())?;

    // Content
    match options.endianness {
        Endianness::LittleEndian => {
            write_multi_line_string_content::<LittleEndian>(writer, geom, options)
        }
        Endianness::BigEndian => {
            write_multi_line_string_content::<BigEndian>(writer, geom, options)
        }
    }
}

fn write_multi_line_string_content<B: ByteOrder>(
    writer: &mut impl Write,
    geom: &impl MultiLineStringTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    let wkb_type = WkbType::MultiLineString(geom.dim().try_into()?);
    writer.write_u32::<B>(wkb_type.into())?;

    // numPoints
    writer.write_u32::<B>(geom.num_line_strings().try_into()?)?;

    for line_string in geom.line_strings() {
        write_line_string(writer, &line_string, options)?;
    }

    Ok(())
}
