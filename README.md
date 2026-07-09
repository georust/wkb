# `wkb`

[![Crate][crates-badge]][crates-url]
[![API Documentation][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/wkb.svg
[crates-url]: https://crates.io/crates/wkb
[docs-badge]: https://docs.rs/wkb/badge.svg
[docs-url]: https://docs.rs/wkb

A fast, freely-licensed implementation of reading and writing the [Well-Known Binary][wkb] encoding of vector geometries.

## Features

- Reading and write without copying to an intermediate representation, thanks to [`geo_traits`][geo_traits].
- Full support for Z, M, and ZM dimension data.
- Full support for little-endian and big-endian data, in both reading and writing.
- Read-only support for extended Well-Known Binary (EWKB). Any embedded SRID is currently ignored.
- MIT and Apache 2 license.

[geo_traits]: https://docs.rs/geo-traits/latest/geo_traits/
[wkb]: https://libgeos.org/specifications/wkb/

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
