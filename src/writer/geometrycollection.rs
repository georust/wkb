use crate::common::WkbType;
use crate::error::WkbResult;
use crate::writer::geometry::{geometry_wkb_size, write_geometry};
use crate::writer::WriteOptions;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::GeometryCollectionTrait;
use std::io::Write;

/// The number of bytes this GeometryCollection will take up when encoded as WKB
pub fn geometry_collection_wkb_size(geom: &impl GeometryCollectionTrait<T = f64>) -> usize {
    let mut sum = 1 + 4 + 4;

    for inner_geom in geom.geometries() {
        sum += geometry_wkb_size(&inner_geom);
    }

    sum
}

/// Write a GeometryCollection geometry to a Writer encoded as WKB
pub fn write_geometry_collection(
    writer: &mut impl Write,
    geom: &impl GeometryCollectionTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    // Byte order
    writer.write_u8(options.endianness.into())?;

    // Content
    match options.endianness {
        Endianness::LittleEndian => {
            write_geometry_collection_content::<LittleEndian>(writer, geom, options)
        }
        Endianness::BigEndian => {
            write_geometry_collection_content::<BigEndian>(writer, geom, options)
        }
    }
}

fn write_geometry_collection_content<B: ByteOrder>(
    writer: &mut impl Write,
    geom: &impl GeometryCollectionTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    let wkb_type = WkbType::GeometryCollection(geom.dim().try_into()?);
    writer.write_u32::<B>(wkb_type.into())?;

    // numGeometries
    writer.write_u32::<B>(geom.num_geometries().try_into().unwrap())?;

    for inner_geom in geom.geometries() {
        write_geometry(writer, &inner_geom, options)?;
    }

    Ok(())
}
