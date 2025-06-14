use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use lifegame_core::World;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::Duration;

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benchmark");
    for size in [100, 500, 1000] {
        let mut rng = StdRng::seed_from_u64(999);
        let alive_prob = 0.2;
        let data = (0..size * size)
            .map(|_| rng.random_bool(alive_prob) as u8)
            .collect::<Vec<_>>();
        group.throughput(Throughput::Bytes(size));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut world = World::new(size as usize, size as usize, &data).unwrap();
            b.iter(|| world.next())
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(15));
    targets = benchmark
}

criterion_main!(benches);
