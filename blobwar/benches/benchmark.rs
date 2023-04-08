/// Measures mean time for the same set of serialized game states
use std::{fs, io::Read};

use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{
    AlphaBeta, AlphaBetaPass, AlphaBetaTable, BenchmarkUnitaire, MinMax, Strategy,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

static MIN_DEPTH: u8 = 1;
static MAX_DEPTH: u8 = 3;

// Do benchmark on all strategies
fn general_benchmark<T: BenchmarkUnitaire>(c: &mut Criterion, name: &str) {
    let mut group = c.benchmark_group("Benchmark");
    for size in MIN_DEPTH..=MAX_DEPTH {
        group.throughput(criterion::Throughput::Bytes(size as u64));
        group.bench_with_input(format!("{name}"), &size, |b, &s| {
            b.iter(|| {
                let strat = T::new(s);
                let board = Default::default();
                let mut game = black_box(Configuration::new(&board));
                game.battle(strat, MinMax(2))
            })
        });
    }
    group.finish();
}

fn alphabeta(c: &mut Criterion) {
    general_benchmark::<AlphaBeta>(c, "Sequential Alphabeta");
}

fn alphabetapass(c: &mut Criterion) {
    general_benchmark::<AlphaBetaPass>(c, "Alphabeta with pass");
}

fn _minmax(c: &mut Criterion) {
    general_benchmark::<MinMax>(c, "Minmax");
}

fn alphabetatable(c: &mut Criterion) {
    general_benchmark::<AlphaBetaTable>(c, "Alphabeta with table");
}

criterion_group!(benchmark, /* alphabetapass,  */alphabeta, _minmax /* alphabetatable */);
criterion_main!(benchmark);
