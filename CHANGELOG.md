# Changes

## Unreleased

- Don't panic when parsing invalid WKB (#74).
- Fix CI by removing georust container & fix clippy lint (#140)
- Remove trait wrappers to work around Rust 1.90 compiler regression (#77)

## 0.9.0 - 2025-05-14

- **BREAKING**: Standardize capitalization of `Wkb` in the codebase.
  - `WKBResult` is now `WkbResult`.
  - `WKBError` is now `WkbError`.
- **BREAKING**: Bump to geo-traits 0.3.
- **BREAKING**: Change the signature of writer functions to accept a `WriterOptions` object instead of `Endianness`. This allows a future update to write an SRID value without requiring a breaking change in the future.
- Expose `GeometryType` and `Dimension` from parsed `Wkb` object (#65).
- Expose `wkb::reader::Wkb` type through public API.
- Ensure correct dimension when writing `Rect` to WKB. (#67)
- Make lifetime annotations of `Wkb` more permissive. (#59)
- Define associated types as references for geo-traits implementations of MultiLineString, Polygon and MultiPolygon to avoid creating unnecessary copies. (#61)
- Make lifetime annotations of specialized `GeometryTrait` implementations more permissive. (#63)

## 0.8.0 - 2024-12-03

- As of this version the `wkb` crate is an entirely new implementation of reading/writing WKB. Previous versions of the `wkb` crate were published from https://github.com/amandasaurus/rust-wkb.

