use criterion::{
    black_box, criterion_group, criterion_main, Bencher, Criterion, ParameterizedBenchmark,
    Throughput,
};
use longest_increasing_subsequence::lis_with;
use rand::{seq::SliceRandom, SeedableRng};

fn bench_over_inputs(xs: &[u32], b: &mut Bencher) {
    let mut predecessors = vec![0; xs.len()];
    let mut starts = vec![0; xs.len()];
    let mut results = Vec::with_capacity(xs.len());
    b.iter(|| {
        let xs = black_box(xs);
        results.clear();
        lis_with(
            xs,
            &mut results,
            |a, b| a < b,
            &mut predecessors,
            &mut starts,
        );
        black_box(&results);
        black_box(&predecessors);
        black_box(&starts);
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "lis_with",
        ParameterizedBenchmark::new(
            "in_order",
            |b, n| {
                let xs = (0_u32..*n).collect::<Vec<_>>();
                bench_over_inputs(&xs, b);
            },
            (20..=100).step_by(20).map(|n| n * n).collect::<Vec<u32>>(),
        )
        .throughput(|n| Throughput::Elements(*n)),
    );

    c.bench(
        "lis_with",
        ParameterizedBenchmark::new(
            "reverse_order",
            |b, n| {
                let mut xs = (0_u32..*n).collect::<Vec<_>>();
                xs.reverse();
                bench_over_inputs(&xs, b);
            },
            (20..=100).step_by(20).map(|n| n * n).collect::<Vec<u32>>(),
        )
        .throughput(|n| Throughput::Elements(*n)),
    );

    c.bench(
        "lis_with",
        ParameterizedBenchmark::new(
            "random_order",
            |b, n| {
                let mut xs = (0_u32..*n).collect::<Vec<_>>();

                // NB: Use same seed every time so benchmark results are
                // comparable.
                let mut rng = rand::rngs::SmallRng::seed_from_u64(1337);
                xs.shuffle(&mut rng);

                bench_over_inputs(&xs, b);
            },
            (20..=100).step_by(20).map(|n| n * n).collect::<Vec<u32>>(),
        )
        .throughput(|n| Throughput::Elements(*n)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
