use blobwar::{
    configuration::Configuration,
    strategy::{AlphaBeta, Greedy, MinMax},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn _benchmark(c: &mut Criterion) {
    let board = Default::default();
    let mut game = black_box(Configuration::new(&board));
    c.bench_function("minmax algorithm", |b| {
            b.iter(|| game.battle(MinMax(1), Greedy()))
        });
    }

criterion_group!(benches, _benchmark);
criterion_main!(benches);
