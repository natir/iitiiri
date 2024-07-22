//! Compare reverse complement function

/* std use */

/* crate use */
use rand::Rng as _;
use rand::SeedableRng as _;

/* project use */
use iitiiri::node;

use criterion::criterion_group;

pub fn build(c: &mut criterion::Criterion) {
    let mut g = c.benchmark_group("build");

    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    let pos_range = 0..100_000;
    let range_len = 0..200;
    let num_interval = 10_000;

    let nodes = (0..num_interval)
        .map(|_| {
            let a = rng.gen_range(pos_range.clone());
            let b = a + rng.gen_range(range_len.clone());

            node::Node::new(a, b, (a, b))
        })
        .collect::<Vec<node::Node<usize, (usize, usize)>>>();

    g.bench_function("iit", |b| {
        b.iter(|| criterion::black_box(iitiiri::Iit::new(nodes.clone())))
    });

    seq_macro::seq!(I in 0..6 {
        g.bench_with_input(criterion::BenchmarkId::new("iitii", 1 << I), &(1 << I), |b, _| {
            b.iter(|| criterion::black_box(iitiiri::Iitii::<usize, (usize, usize), {1 << I}>::new(nodes.clone())))
        });
    });

    g.finish()
}

pub fn overlap(c: &mut criterion::Criterion) {
    let mut g = c.benchmark_group("overlap");

    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    let pos_range = 0..100_000;
    let range_len = 0..200;
    let num_interval = 10_000;

    let nodes = (0..num_interval)
        .map(|_| {
            let a = rng.gen_range(pos_range.clone());
            let b = a + rng.gen_range(range_len.clone());

            node::Node::new(a, b, (a, b))
        })
        .collect::<Vec<node::Node<usize, (usize, usize)>>>();

    g.bench_function("iit", |b| {
        let iit = iitiiri::Iit::new(nodes.clone());

        let start = rng.gen_range(pos_range.clone());
        let stop = start + rng.gen_range(0..2000);

        b.iter(|| criterion::black_box(iit.overlap(start, stop)))
    });

    seq_macro::seq!(I in 0..6 {
    g.bench_with_input(criterion::BenchmarkId::new("iitii", 1 << I), &(1 << I), |b, _| {
        let iit = iitiiri::Iitii::<usize, (usize, usize), {1 << I}>::new(nodes.clone());

            let start = rng.gen_range(pos_range.clone());
            let stop = start + rng.gen_range(0..2000);


            b.iter(|| criterion::black_box(iit.overlap(start, stop)))
       });
    });

    g.finish()
}

pub fn setup(c: &mut criterion::Criterion) {
    build(c);
    overlap(c);
}

criterion::criterion_group! {
name = benches;
config = criterion::Criterion::default()               // Check https://docs.rs/criterion/latest/criterion/struct.Criterion.html
    .warm_up_time(core::time::Duration::from_secs(1))  // for full parameter
.measurement_time(core::time::Duration::from_secs(5))
.sample_size(500);
targets = setup
}
criterion::criterion_main!(benches);
