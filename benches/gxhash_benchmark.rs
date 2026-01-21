use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

// Import from gxhash_core which is the actual gxhash crate (renamed in Cargo.toml)
use gxhash_core::{gxhash32, gxhash64, gxhash128};

fn bench_gxhash32(c: &mut Criterion) {
    let mut group = c.benchmark_group("gxhash32");
    
    for size in [16, 64, 256, 1024, 4096, 16384].iter() {
        let data = vec![0u8; *size];
        group.throughput(Throughput::Bytes(*size as u64));
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                black_box(gxhash32(black_box(&data), black_box(0)))
            });
        });
    }
    
    group.finish();
}

fn bench_gxhash64(c: &mut Criterion) {
    let mut group = c.benchmark_group("gxhash64");
    
    for size in [16, 64, 256, 1024, 4096, 16384].iter() {
        let data = vec![0u8; *size];
        group.throughput(Throughput::Bytes(*size as u64));
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                black_box(gxhash64(black_box(&data), black_box(0)))
            });
        });
    }
    
    group.finish();
}

fn bench_gxhash128(c: &mut Criterion) {
    let mut group = c.benchmark_group("gxhash128");
    
    for size in [16, 64, 256, 1024, 4096, 16384].iter() {
        let data = vec![0u8; *size];
        group.throughput(Throughput::Bytes(*size as u64));
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                black_box(gxhash128(black_box(&data), black_box(0)))
            });
        });
    }
    
    group.finish();
}

fn bench_small_inputs(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_inputs");
    
    let small_data = b"Hello";
    group.throughput(Throughput::Bytes(small_data.len() as u64));
    
    group.bench_function("gxhash32_small", |b| {
        b.iter(|| black_box(gxhash32(black_box(small_data), black_box(0))))
    });
    
    group.bench_function("gxhash64_small", |b| {
        b.iter(|| black_box(gxhash64(black_box(small_data), black_box(0))))
    });
    
    group.bench_function("gxhash128_small", |b| {
        b.iter(|| black_box(gxhash128(black_box(small_data), black_box(0))))
    });
    
    group.finish();
}

criterion_group!(benches, bench_gxhash32, bench_gxhash64, bench_gxhash128, bench_small_inputs);
criterion_main!(benches);
