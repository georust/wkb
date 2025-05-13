use std::io::Write;

use geo_traits::{LineStringTrait, LineTrait};

use crate::error::WkbResult;
use crate::writer::{line_string_wkb_size, write_line_string, WriteOptions};

/// A wrapper around an impl LineTrait to provide LineStringTrait
struct LineWrapper<'a, G: LineTrait<T = f64>>(&'a G);

impl<'a, G: LineTrait<T = f64>> LineStringTrait for LineWrapper<'a, G> {
    type T = f64;
    type CoordType<'b>
        = G::CoordType<'a>
    where
        G: 'b,
        Self: 'b;

    fn dim(&self) -> geo_traits::Dimensions {
        self.0.dim()
    }

    fn num_coords(&self) -> usize {
        2
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        match i {
            0 => self.0.start(),
            1 => self.0.end(),
            _ => unreachable!(),
        }
    }
}

/// The number of bytes this Line will take up when encoded as WKB
pub fn line_wkb_size(geom: &impl LineTrait<T = f64>) -> usize {
    line_string_wkb_size(&LineWrapper(geom))
}

/// Write a Line geometry to a Writer encoded as WKB
pub fn write_line(
    writer: &mut impl Write,
    geom: &impl LineTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    write_line_string(writer, &LineWrapper(geom), options)
}
