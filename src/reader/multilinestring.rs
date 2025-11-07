use std::io::Cursor;

use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::linestring::LineString;
use crate::reader::util::{has_srid, ReadBytesExt};
use crate::reader::HEADER_BYTES;
use crate::Endianness;
use geo_traits::MultiLineStringTrait;

/// A WKB MultiLineString
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
#[derive(Debug, Clone)]
pub struct MultiLineString<'a> {
    /// A LineString object for each of the internal line strings
    wkb_line_strings: Vec<LineString<'a>>,
    buf: &'a [u8],
    dim: Dimension,
}

impl<'a> MultiLineString<'a> {
    pub(crate) fn try_new(
        buf: &'a [u8],
        byte_order: Endianness,
        dim: Dimension,
    ) -> WkbResult<Self> {
        let has_srid = has_srid(buf, byte_order)?;
        let num_line_strings_offset = HEADER_BYTES + if has_srid { 4 } else { 0 };

        let mut reader = Cursor::new(buf);
        reader.set_position(num_line_strings_offset);
        let num_line_strings = reader
            .read_u32(byte_order)?
            .try_into()
            .map_err(|e| WkbError::General(format!("Invalid number of line strings: {}", e)))?;

        let mut line_string_offset = num_line_strings_offset + 4;

        let mut wkb_line_strings = Vec::with_capacity(num_line_strings);
        for _ in 0..num_line_strings {
            let ls = LineString::try_new(&buf[line_string_offset as usize..], byte_order, dim)?;
            line_string_offset += ls.size();
            wkb_line_strings.push(ls);
        }

        Ok(Self {
            wkb_line_strings,
            buf: &buf[0..line_string_offset as usize],
            dim,
        })
    }

    /// The number of bytes in this object, including any header
    #[inline]
    pub fn size(&self) -> u64 {
        self.buf.len() as u64
    }

    /// The dimension of this MultiLineString
    pub fn dimension(&self) -> Dimension {
        self.dim
    }

    /// Get the underlying buffer of this MultiLineString
    #[inline]
    pub fn buf(&self) -> &'a [u8] {
        self.buf
    }
}

impl<'a> MultiLineStringTrait for MultiLineString<'a> {
    type InnerLineStringType<'b>
        = &'b LineString<'a>
    where
        Self: 'b;

    fn num_line_strings(&self) -> usize {
        self.wkb_line_strings.len()
    }

    unsafe fn line_string_unchecked(&self, i: usize) -> Self::InnerLineStringType<'_> {
        self.wkb_line_strings.get_unchecked(i)
    }
}

impl<'a, 'b> MultiLineStringTrait for &'b MultiLineString<'a> {
    type InnerLineStringType<'c>
        = &'b LineString<'a>
    where
        Self: 'c;

    fn num_line_strings(&self) -> usize {
        self.wkb_line_strings.len()
    }

    unsafe fn line_string_unchecked(&self, i: usize) -> Self::InnerLineStringType<'_> {
        self.wkb_line_strings.get_unchecked(i)
    }
}
