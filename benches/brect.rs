use criterion::{criterion_group, criterion_main};
use geo_traits::{CoordTrait, GeometryTrait, GeometryType, LineStringTrait, PolygonTrait};
use geo_types::{coord, Geometry};
use wkt::TryFromWkt;

fn load_small_wkt() -> Geometry {
    let s = include_str!("./small.wkt");
    Geometry::try_from_wkt_str(s).unwrap()
}

fn to_wkb(geom: &Geometry) -> Vec<u8> {
    let mut buffer = Vec::new();
    wkb::writer::write_geometry(&mut buffer, geom, &Default::default()).unwrap();
    buffer
}

fn bench_brect_small(c: &mut criterion::Criterion) {
    let small = load_small_wkt();
    let small_wkb = to_wkb(&small);
    let wkb_geom = wkb::reader::read_wkb(&small_wkb).unwrap();
    let wkb_polygon = if let GeometryType::Polygon(polygon) = wkb_geom.as_type() {
        polygon
    } else {
        panic!("expected a polygon");
    };

    c.bench_function("brect small", |bencher| {
        bencher.iter(|| {
            let exterior = wkb_polygon.exterior().unwrap();
            let mut min_x = f64::MAX;
            let mut min_y = f64::MAX;
            let mut max_x = f64::MIN;
            let mut max_y = f64::MIN;
            for coord in exterior.coords() {
                min_x = min_x.min(coord.x());
                min_y = min_y.min(coord.y());
                max_x = max_x.max(coord.x());
                max_y = max_y.max(coord.y());
            }
            let rect =
                geo_types::Rect::new(coord! { x: min_x, y: min_y }, coord! { x: max_x, y: max_y });
            criterion::black_box(rect);
        });
    });
}

criterion_group!(benches, bench_brect_small);
criterion_main!(benches);
