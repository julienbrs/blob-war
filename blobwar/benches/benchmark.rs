/// Measures mean time for the same set of serialized game states
use std::{fs, io::Read};

use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{AlphaBeta, NewUniStrategy, ParAlphaBeta, Strategy, MinMax};
use criterion::{criterion_group, criterion_main, Criterion};

static MIN_DEPTH: u8 = 4;
static MAX_DEPTH: u8 = 6;

fn make_move<T: Strategy>(mut strat: T) {
    for entry in fs::read_dir("./serializations").expect("Unable to read directory") {
        let entry = entry.expect("Unable to read file");
        let path = entry.path();

        // Do not go inside subfolders
        if let Ok(mut file) = fs::File::open(&path) {
            let mut serialization = String::new();
            let _ = file.read_to_string(&mut serialization);
            let board = Board::deserialize(&serialization);
            let conf = Configuration::deserialize(&serialization, &board);

            strat.compute_next_move(&conf);
        }
    }
}

fn general_benchmark<T: NewUniStrategy>(c: &mut Criterion, name: &str) {
    let mut group = c.benchmark_group(name);
    for size in MIN_DEPTH..=MAX_DEPTH {
        group.throughput(criterion::Throughput::Bytes(size as u64));
        group.bench_with_input(format!("{size}"), &size, |b, &s| {
            b.iter(|| {
                let strat = T::new(s);
                make_move(strat);
            })
        });
    }
    group.finish();
}

fn alphabeta(c: &mut Criterion) {
    general_benchmark::<AlphaBeta>(c, "Sequential Alphabeta");
}

fn paralphabeta(c: &mut Criterion) {
    general_benchmark::<ParAlphaBeta>(c, "Parallel Alphabeta");
}

// Too slow to even test
fn _minmax(c: &mut Criterion) {
    general_benchmark::<MinMax>(c, "Minmax");
}

criterion_group!(time, paralphabeta, alphabeta);
criterion_main!(time);