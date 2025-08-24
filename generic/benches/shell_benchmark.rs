use criterion::{Criterion, criterion_group, criterion_main};
use generic::{Shell, execute_boxed_trait_object, execute_generics, execute_trait_object};
use std::hint::black_box;

pub fn generics_benchmark(c: &mut Criterion) {
    c.bench_function("generics", |b| {
        b.iter(|| {
            let cmd = Shell::new("ls", &[]);
            execute_generics(black_box(&cmd)).unwrap();
        })
    });
}

pub fn trait_object_benchmark(c: &mut Criterion) {
    c.bench_function("trait object", |b| {
        b.iter(|| {
            let cmd = Shell::new("ls", &[]);
            execute_trait_object(black_box(&cmd)).unwrap();
        })
    });
}

pub fn boxed_trait_object_benchmark(c: &mut Criterion) {
    c.bench_function("boxed trait object", |b| {
        b.iter(|| {
            let cmd = Shell::new("ls", &[]);
            execute_boxed_trait_object(black_box(Box::new(cmd))).unwrap();
        })
    });
}

criterion_group!(
    shell_benches,
    generics_benchmark,
    trait_object_benchmark,
    boxed_trait_object_benchmark
);

criterion_main!(shell_benches);
