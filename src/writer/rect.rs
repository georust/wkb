use crate::common::WkbType;
use crate::error::WkbResult;
use crate::writer::WriteOptions;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::{CoordTrait, RectTrait};
use std::io::Write;

/// The number of bytes this Rect will take up when encoded as WKB
///
/// Note that only 2D Rects are supported. Even if the input Rect has more than 2 dimensions, only
/// the X and Y dimensions will be written.
pub fn rect_wkb_size(geom: &impl RectTrait<T = f64>) -> usize {
    let header = 1 + 4 + 4;
    let each_coord = geom.dim().size() * 8;
    let all_coords = 5 * each_coord;
    header + all_coords
}

/// Write a Rect geometry to a Writer encoded as WKB
///
/// Note that only 2D Rects are supported. Even if the input Rect has more than 2 dimensions, only
/// the X and Y dimensions will be written.
pub fn write_rect(
    writer: &mut impl Write,
    geom: &impl RectTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    // Byte order
    writer.write_u8(options.endianness.into())?;

    // Content
    match options.endianness {
        Endianness::LittleEndian => write_rect_content::<LittleEndian>(writer, geom),
        Endianness::BigEndian => write_rect_content::<BigEndian>(writer, geom),
    }
}

/// Minimal struct to hold a named coordinate pair
struct Coord {
    x: f64,
    y: f64,
}

fn write_rect_content<B: ByteOrder>(
    writer: &mut impl Write,
    geom: &impl RectTrait<T = f64>,
) -> WkbResult<()> {
    let wkb_type = WkbType::Polygon(geom.dim().try_into()?);
    writer.write_u32::<B>(wkb_type.into())?;

    // numRings
    let num_rings = 1;
    writer.write_u32::<B>(num_rings)?;

    let min_coord = geom.min();
    let max_coord = geom.max();

    let ll = Coord {
        x: min_coord.x(),
        y: min_coord.y(),
    };
    let ul = Coord {
        x: min_coord.x(),
        y: max_coord.y(),
    };
    let ur = Coord {
        x: max_coord.x(),
        y: max_coord.y(),
    };
    let lr = Coord {
        x: max_coord.x(),
        y: min_coord.y(),
    };

    writer.write_f64::<B>(ll.x)?;
    writer.write_f64::<B>(ll.y)?;

    writer.write_f64::<B>(ul.x)?;
    writer.write_f64::<B>(ul.y)?;

    writer.write_f64::<B>(ur.x)?;
    writer.write_f64::<B>(ur.y)?;

    writer.write_f64::<B>(lr.x)?;
    writer.write_f64::<B>(lr.y)?;

    writer.write_f64::<B>(ll.x)?;
    writer.write_f64::<B>(ll.y)?;

    Ok(())
}
