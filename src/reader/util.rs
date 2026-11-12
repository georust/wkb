use byteorder::{BigEndian, LittleEndian};

use crate::reader::HEADER_BYTES;
use crate::{common::WkbGeometryCode, error::WkbError, Endianness};
use std::io::{Error, ErrorKind};

/// Smallest on-wire size of a full WKB sub-geometry: a 5-byte header (byte order + geometry
/// type) followed by a 4-byte element count.
pub(crate) const MIN_WKB_GEOMETRY_SIZE: usize = HEADER_BYTES as usize + 4;

/// Smallest on-wire size of a linear ring: a 4-byte point count.
pub(crate) const MIN_LINEAR_RING_SIZE: usize = 4;

/// Cap a `Vec` pre-allocation driven by an untrusted element count so a malicious count field
/// can't request gigabytes of memory before a single element is read.
///
/// Every element occupies at least `min_element_size` bytes, so `remaining` bytes can hold at
/// most `remaining / min_element_size` of them. A valid geometry always has
/// `remaining >= count * min_element_size`, so `count` is returned unchanged for valid input.
pub(crate) fn bounded_capacity(count: usize, remaining: usize, min_element_size: usize) -> usize {
    count.min(remaining / min_element_size)
}

pub(crate) trait ReadBytesExt: byteorder::ReadBytesExt {
    fn read_f64(&mut self, byte_order: Endianness) -> Result<f64, Error> {
        match byte_order {
            Endianness::BigEndian => byteorder::ReadBytesExt::read_f64::<BigEndian>(self),
            Endianness::LittleEndian => byteorder::ReadBytesExt::read_f64::<LittleEndian>(self),
        }
    }
}

/// All types that implement `Read` get methods defined in `ReadBytesExt`
/// for free.
impl<R: std::io::Read + ?Sized> ReadBytesExt for R {}

/// Read the `u32` at `offset` of `buf`, without the overhead of a [`std::io::Cursor`].
#[inline]
pub(crate) fn read_u32_at(buf: &[u8], offset: u64, byte_order: Endianness) -> Result<u32, Error> {
    let Some(bytes) = usize::try_from(offset)
        .ok()
        .and_then(|start| buf.get(start..start.checked_add(4)?))
    else {
        return Err(unexpected_eof());
    };
    let bytes: [u8; 4] = bytes.try_into().unwrap();
    Ok(match byte_order {
        Endianness::BigEndian => u32::from_be_bytes(bytes),
        Endianness::LittleEndian => u32::from_le_bytes(bytes),
    })
}

/// The error a [`std::io::Cursor`] gives when the buffer ends too soon.
#[cold]
pub(crate) fn unexpected_eof() -> Error {
    Error::new(ErrorKind::UnexpectedEof, "failed to fill whole buffer")
}

/// Return `true` if this WKB item is EWKB and has an embedded SRID
#[inline]
pub(crate) fn has_srid(buf: &[u8], byte_order: Endianness) -> Result<bool, WkbError> {
    // Skip the 1-byte byte order that we already know
    Ok(WkbGeometryCode::new(read_u32_at(buf, 1, byte_order)?).has_srid())
}

#[cfg(test)]
mod tests {
    use super::{bounded_capacity, MIN_WKB_GEOMETRY_SIZE};

    #[test]
    fn bounded_capacity_caps_untrusted_count() {
        // A huge count with an empty or short buffer must not become a huge reservation.
        assert_eq!(
            bounded_capacity(u32::MAX as usize, 0, MIN_WKB_GEOMETRY_SIZE),
            0
        );
        assert_eq!(
            bounded_capacity(u32::MAX as usize, 100, MIN_WKB_GEOMETRY_SIZE),
            11
        );
    }

    #[test]
    fn bounded_capacity_preserves_valid_count() {
        // A valid buffer has room for every declared element, so the count is returned as-is.
        let count = 1000;
        assert_eq!(bounded_capacity(count, 9000, MIN_WKB_GEOMETRY_SIZE), count);
    }
}
