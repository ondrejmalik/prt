use criterion::{black_box, criterion_group, criterion_main, Criterion};

use simd_benchmark::simd::{square, square_basic};

fn criterion_benchmark(c: &mut Criterion) {
    let size = 100_000;
    let mut a1 = vec![0i32; size];
    for i in 0..size {
        a1[i] = i as i32;
    }

    let mut b1 = vec![0i32; size];
    for i in 0..size {
        b1[i] = (size - i) as i32;
    }
    let mut result1 = vec![0i32; size];

    let mut a2 = a1.clone();
    let b2 = a2.clone();
    let mut result2 = result1.clone();
    c.bench_function("square", |ben| unsafe {
        black_box(ben.iter(|| square(&a1, &b1, &mut result1)));
    });

    c.bench_function("square_basic", |ben| unsafe {
        black_box(ben.iter(|| square_basic(&a2, &b2, &mut result2)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
