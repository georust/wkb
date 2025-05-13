# Changes

## Unreleased

- Your change here.

## 0.9.0 - unreleased

- **BREAKING**: Standardize capitalization of `Wkb` in the codebase.
  - `WKBResult` is now `WkbResult`.
  - `WKBError` is now `WkbError`.
- Expose `wkb::reader::Wkb` type through public API.
- Make lifetime annotations of `Wkb` more permissive. (#59)
- Define associated types as references for geo-traits implementations of MultiLineString, Polygon and MultiPolygon to avoid creating unnecessary copies. (#61)
- Make lifetime annotations of specialized `GeometryTrait` implementations more permissive. (#63)

## 0.8.0 - 2024-12-03

- As of this version the `wkb` crate is an entirely new implementation of reading/writing WKB. Previous versions of the `wkb` crate were published from https://github.com/amandasaurus/rust-wkb.

