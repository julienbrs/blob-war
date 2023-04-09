/// Measures mean benchmark for the same set of serialized game states
use std::{fs, io::Read};

use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{
    AlphaBeta, AlphaBetaPass, AlphaBetaTable, BenchmarkUnitaire, MinMax, Strategy,
};
use criterion::measurement::WallTime;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkGroup, Criterion};

static MIN_DEPTH: u8 = 3;
static MAX_DEPTH: u8 = 6;

// Do benchmark on all strategies
fn benchmark_per_group<T: Strategy>(
    group: &mut  BenchmarkGroup<WallTime>,
    name: &str,
    createfunc: fn(u8) -> T,
) {
    for size in MIN_DEPTH..=MAX_DEPTH {
        group.throughput(criterion::Throughput::Bytes(size as u64));
        group.bench_with_input(name, &size, |b, &s| {
            b.iter(|| {
                let strat = createfunc(s);
                let board = Default::default();
                let mut game = black_box(Configuration::new(&board));
                game.battle_no_log(strat, MinMax(3))
            })
        });
    }
}

/// Benchmark all strategies which are in criterion_group using benchmark per strategy
fn general_benchmark(c: &mut Criterion) {
    let mut total_group = c.benchmark_group("Benchmark all strategies");

    // call benchmark_per_group for each strategy
    benchmark_per_group(&mut total_group, "AlphaBeta", |x| AlphaBeta::new(x));
    benchmark_per_group(&mut total_group, "AlphaBetaPass", |x| AlphaBeta::new(x));
    benchmark_per_group(&mut total_group, "AlphaBetaTable", |x| AlphaBeta::new(x));
    benchmark_per_group(&mut total_group, "MinMax", |x| AlphaBeta::new(x));
    total_group.finish(); // finish the benchmark group and plot the results
}
criterion_group!(benchmark, general_benchmark);
criterion_main!(benchmark);
