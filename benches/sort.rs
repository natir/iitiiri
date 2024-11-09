//! Benchmark different sort method

/* std use */

/* crate use */
use rand::Rng as _;
use rand::SeedableRng as _;

/* project use */

fn random(c: &mut criterion::Criterion) {
    let mut g = c.benchmark_group("sort_random");

    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    let pos_range = 0..100_000;
    let range_len = 0..200;
    let num_interval: u64 = 1 << 20;

    let mut values = (0..num_interval)
        .map(|_| {
            let a = rng.gen_range(pos_range.clone());
            let b = a + rng.gen_range(range_len.clone());

            clairiere::node::Node::new(a, b, true)
        })
        .collect::<Vec<clairiere::node::Node<usize, bool>>>();

    g.bench_function("stable", |b| {
        b.iter(|| {
            criterion::black_box(values.sort());
        });
    });
    g.bench_function("unstable", |b| {
        b.iter(|| {
            criterion::black_box(values.sort_unstable());
        });
    });
    g.bench_function("stable_by_key", |b| {
        b.iter(|| {
            criterion::black_box(values.sort_by_key(|x| *x.start()));
        });
    });
    g.bench_function("unstable_by_key", |b| {
        b.iter(|| {
            criterion::black_box(values.sort_unstable_by_key(|x| *x.start()));
        });
    });
}

fn realist(c: &mut criterion::Criterion) {
    let mut g = c.benchmark_group("sort_realist");

    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    let pos_range = 0..100_000;
    let range_len = 0..200;
    let num_interval: u64 = 1 << 20;
    let num_of_group: u64 = 1 << 8;
    let obj_in_group = num_interval / num_of_group;

    let mut values = Vec::new();
    for _ in 0..num_of_group {
        let mut sub_part = (0..obj_in_group)
            .map(|_| {
                let a = rng.gen_range(pos_range.clone());
                let b = a + rng.gen_range(range_len.clone());

                clairiere::node::Node::new(a, b, true)
            })
            .collect::<Vec<clairiere::node::Node<usize, bool>>>();

        sub_part.sort_unstable();
        values.extend(sub_part);
    }

    g.bench_function("stable", |b| {
        b.iter(|| {
            criterion::black_box(values.sort());
        });
    });
    g.bench_function("unstable", |b| {
        b.iter(|| {
            criterion::black_box(values.sort_unstable());
        });
    });
    g.bench_function("stable_by_key", |b| {
        b.iter(|| {
            criterion::black_box(values.sort_by_key(|x| *x.start()));
        })
    });
    g.bench_function("unstable_by_key", |b| {
        b.iter(|| {
            criterion::black_box(values.sort_unstable_by_key(|x| *x.start()));
        });
    });
}

fn setup(c: &mut criterion::Criterion) {
    random(c);
    realist(c);
}

criterion::criterion_group! {
    name = benches;
    config = criterion::Criterion::default()
    .warm_up_time(core::time::Duration::from_secs(1))
    .measurement_time(core::time::Duration::from_secs(3))
    .sample_size(500);
    targets = setup
}
criterion::criterion_main!(benches);
