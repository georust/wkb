use std::io::Write;

use geo_traits::{LineStringTrait, PolygonTrait, TriangleTrait};

use crate::error::WkbResult;
use crate::writer::{polygon_wkb_size, write_polygon, WriteOptions};

/// A wrapper around an impl TriangleTrait to provide LineStringTrait and PolygonTrait
struct TriangleWrapper<'a, G: TriangleTrait<T = f64>>(&'a G);

impl<'a, G: TriangleTrait<T = f64>> LineStringTrait for &'a TriangleWrapper<'a, G> {
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
        3
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        match i {
            0 => self.0.first(),
            1 => self.0.second(),
            2 => self.0.third(),
            _ => unreachable!(),
        }
    }
}

impl<G: TriangleTrait<T = f64>> PolygonTrait for TriangleWrapper<'_, G> {
    type T = f64;
    type RingType<'b>
        = &'b TriangleWrapper<'b, G>
    where
        G: 'b,
        Self: 'b;

    fn dim(&self) -> geo_traits::Dimensions {
        self.0.dim()
    }

    fn exterior(&self) -> Option<Self::RingType<'_>> {
        Some(self)
    }

    fn num_interiors(&self) -> usize {
        0
    }

    unsafe fn interior_unchecked(&self, _i: usize) -> Self::RingType<'_> {
        unreachable!()
    }
}

/// The number of bytes this Triangle will take up when encoded as WKB
pub fn triangle_wkb_size(geom: &impl TriangleTrait<T = f64>) -> usize {
    polygon_wkb_size(&TriangleWrapper(geom))
}

/// Write a Triangle geometry to a Writer encoded as WKB
pub fn write_triangle(
    writer: &mut impl Write,
    geom: &impl TriangleTrait<T = f64>,
    options: &WriteOptions,
) -> WkbResult<()> {
    write_polygon(writer, &TriangleWrapper(geom), options)
}
