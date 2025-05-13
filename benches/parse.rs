use criterion::{criterion_group, criterion_main};
use geo_traits::to_geo::ToGeoGeometry;
use geo_types::Geometry;
use wkt::TryFromWkt;

fn load_small_wkt() -> Geometry {
    let s = include_str!("./small.wkt");
    Geometry::try_from_wkt_str(s).unwrap()
}

fn load_big_wkt() -> Geometry {
    let s = include_str!("./big.wkt");
    Geometry::try_from_wkt_str(s).unwrap()
}

fn to_wkb(geom: &Geometry) -> Vec<u8> {
    let mut buffer = Vec::new();
    wkb::writer::write_geometry(&mut buffer, geom, &Default::default()).unwrap();
    buffer
}

fn bench_parse(c: &mut criterion::Criterion) {
    let small = load_small_wkt();
    let big = load_big_wkt();
    let small_wkb = to_wkb(&small);
    let big_wkb = to_wkb(&big);

    c.bench_function("parse small", |bencher| {
        bencher.iter(|| {
            let _ = wkb::reader::read_wkb(&small_wkb).unwrap();
        });
    });

    c.bench_function("parse big", |bencher| {
        bencher.iter(|| {
            let _ = wkb::reader::read_wkb(&big_wkb).unwrap();
        });
    });

    c.bench_function("parse small to geo", |bencher| {
        bencher.iter(|| {
            let wkb_geom = wkb::reader::read_wkb(&small_wkb).unwrap();
            let _geo_types_geom = wkb_geom.to_geometry();
        });
    });

    c.bench_function("parse big to geo", |bencher| {
        bencher.iter(|| {
            let wkb_geom = wkb::reader::read_wkb(&big_wkb).unwrap();
            let _geo_types_geom = wkb_geom.to_geometry();
        });
    });

    c.bench_function("encode small", |bencher| {
        bencher.iter(|| {
            let mut buf = Vec::new();
            wkb::writer::write_geometry(&mut buf, &small, &Default::default()).unwrap();
        });
    });

    c.bench_function("encode big", |bencher| {
        bencher.iter(|| {
            let mut buf = Vec::new();
            wkb::writer::write_geometry(&mut buf, &big, &Default::default()).unwrap();
        });
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
