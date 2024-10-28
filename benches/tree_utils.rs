//! Determinate what is the fastest way to detect if index is leaf

/* std use */

/* crate use */

/* project use */

const N: usize = 20;

fn filter_by_level(c: &mut criterion::Criterion) {
    let mut g = c.benchmark_group("filter_by_level");

    let values = (0..1 << N).collect::<Vec<usize>>();

    for level in 0..N {
        g.bench_with_input(
            criterion::BenchmarkId::new("index2level", level),
            &level,
            |b, level| {
                b.iter(|| {
                    criterion::black_box(
                        values
                            .iter()
                            .filter(|index| {
                                clairiere_interpolate::tree_utils::index2level(**index) == *level
                            })
                            .collect::<Vec<&usize>>(),
                    );
                })
            },
        );

        g.bench_with_input(
            criterion::BenchmarkId::new("bits_operation", level),
            &level,
            |b, level| {
                b.iter(|| {
                    criterion::black_box(
                        values
                            .iter()
                            .filter(|index| (*index >> level) & 0b1 == 0)
                            .collect::<Vec<&usize>>(),
                    );
                })
            },
        );
    }
}

fn setup(c: &mut criterion::Criterion) {
    filter_by_level(c);
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
