#[macro_use]
extern crate criterion;
extern crate perch;

use criterion::black_box;
use criterion::Criterion;

use perch::search::search;
use perch::search::SearchType;
use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput-example");

    group.measurement_time(Duration::new(60, 0));
    group.bench_function("HTML Search", |b| {
        b.iter(|| {
            search(
                black_box(String::from("Eliot")),
                black_box(SearchType::HTML),
            )
        })
    });
    group.bench_function("JSON Search", |b| {
        b.iter(|| {
            search(
                black_box(String::from("Eliot")),
                black_box(SearchType::JSON),
            )
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
