use std::io::Write;

use byteorder::{ByteOrder, WriteBytesExt};
use geo_traits::CoordTrait;

use crate::error::WkbResult;

/// Write a coordinate to a Writer encoded as WKB
pub(crate) fn write_coord<B: ByteOrder>(
    writer: &mut impl Write,
    coord: &impl CoordTrait<T = f64>,
) -> WkbResult<()> {
    for i in 0..coord.dim().size() {
        // # Safety
        // We just checked the number of dimensions in this coord
        let val = unsafe { coord.nth_unchecked(i) };
        writer.write_f64::<B>(val)?;
    }

    Ok(())
}
