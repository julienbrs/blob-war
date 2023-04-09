/// Measures mean benchmark for the same set of serialized game states
use std::{fs, io::Read};

use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{
    AlphaBeta, AlphaBetaPass, AlphaBetaTable, BenchmarkUnitaire, MinMax, MinMaxPar, Strategy,
};
use criterion::measurement::WallTime;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkGroup, Criterion};

static MIN_DEPTH: u8 = 1;
static MAX_DEPTH: u8 = 6;

// Do benchmark on all strategies
fn benchmark_per_group<T: Strategy>(
    group: &mut  BenchmarkGroup<WallTime>,
    name: &str,
    createfunc: fn(u8) -> T,
    max_depth: u8,
) {
    for size in MIN_DEPTH..=max_depth {
        group.throughput(criterion::Throughput::Bytes(size as u64));
        group.bench_with_input(name, &size, |b, &s| {
            b.iter(|| {
                let strat = createfunc(s);
                let board = Default::default();
                let mut game = black_box(Configuration::new(&board));
                game.battle_no_log(strat, AlphaBeta(3))
            })
        });
    }
}

/// Benchmark all strategies which are in criterion_group using benchmark per strategy
fn general_benchmark(c: &mut Criterion) {
    let mut total_group = c.benchmark_group("Benchmark all strategies");

    // call benchmark_per_group for each strategy
    benchmark_per_group(&mut total_group, "AlphaBeta", |x| AlphaBeta::new(x), 6);
    benchmark_per_group(&mut total_group, "AlphaBetaPass", |x| AlphaBetaPass::new(x), 6);
    // benchmark_per_group(&mut total_group, "AlphaBetaTable", |x| AlphaBetaTable::new(x), 4);
    // benchmark_per_group(&mut total_group, "MinMax", |x| MinMax::new(x), 4);
    // benchmark_per_group(&mut total_group, "MinMaxPar", |x| MinMaxPar::new(x), 4);

    total_group.finish(); // finish the benchmark group and plot the results
}

criterion_group!(
    name = benchmark;
    config = Criterion::default().sample_size(15);
    targets = general_benchmark
);

criterion_main!(benchmark);
