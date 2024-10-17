// benches/benchmark.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nocoin::transactions::create_transaction;
use nocoin::enhanced_pos::{Validator, select_validator_parallel};
use rand::Rng;

fn benchmark_transaction_creation(c: &mut Criterion) {
    let private_key = vec![1, 2, 3];
    c.bench_function("transaction creation", |b| {
        b.iter(|| {
            create_transaction(
                black_box("Alice".to_string()),
                black_box("Bob".to_string()),
                black_box(100),
                black_box(&private_key),
                black_box(1),
            )
        })
    });
}

fn benchmark_validator_selection(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let validators: Vec<Validator> = (0..1000).map(|_| Validator {
        public_key: vec![rng.gen()],
        stake: rng.gen_range(1..1000),
        reputation_score: rng.gen_range(0.5..1.5),
        carbon_score: rng.gen_range(0.0..0.5),
        slashed: false,
    }).collect();

    c.bench_function("validator selection", |b| {
        b.iter(|| {
            select_validator_parallel(black_box(&validators))
        })
    });
}

criterion_group!(benches, benchmark_transaction_creation, benchmark_validator_selection);
criterion_main!(benches);
