use std::io::Cursor;

use crate::common::Dimension;
use crate::error::{WkbError, WkbResult};
use crate::reader::linestring::LineString;
use crate::reader::util::{has_srid, ReadBytesExt};
use crate::Endianness;
use geo_traits::MultiLineStringTrait;

const HEADER_BYTES: u64 = 5;

/// A WKB MultiLineString
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
#[derive(Debug, Clone)]
pub struct MultiLineString<'a> {
    /// A LineString object for each of the internal line strings
    wkb_line_strings: Vec<LineString<'a>>,
    dim: Dimension,
    has_srid: bool,
}

impl<'a> MultiLineString<'a> {
    pub(crate) fn try_new(
        buf: &'a [u8],
        byte_order: Endianness,
        dim: Dimension,
    ) -> WkbResult<Self> {
        let mut offset = 0;
        let has_srid = has_srid(buf, byte_order, offset)?;
        if has_srid {
            offset += 4;
        }

        let mut reader = Cursor::new(buf);
        reader.set_position(HEADER_BYTES + offset);
        let num_line_strings = reader
            .read_u32(byte_order)?
            .try_into()
            .map_err(|e| WkbError::General(format!("Invalid number of line strings: {}", e)))?;

        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numLineStrings
        let mut line_string_offset = 1 + 4 + 4;
        if has_srid {
            line_string_offset += 4;
        }

        let mut wkb_line_strings = Vec::with_capacity(num_line_strings);
        for _ in 0..num_line_strings {
            let ls = LineString::try_new(buf, byte_order, line_string_offset, dim)?;
            wkb_line_strings.push(ls);
            line_string_offset += ls.size();
        }

        Ok(Self {
            wkb_line_strings,
            dim,
            has_srid,
        })
    }

    /// The number of bytes in this object, including any header
    ///
    /// Note that this is not the same as the length of the underlying buffer
    pub fn size(&self) -> u64 {
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numPoints
        // - Point::size() * self.num_points: the size of each Point for each point
        let mut header = 1 + 4 + 4;
        if self.has_srid {
            header += 4;
        }
        self.wkb_line_strings
            .iter()
            .fold(header, |acc, ls| acc + ls.size())
    }

    pub fn dimension(&self) -> Dimension {
        self.dim
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
