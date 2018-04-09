#[macro_use]
extern crate criterion;
extern crate rand;
extern crate bytecount;

use std::env;
use std::time::Duration;
use rand::Rng;
use criterion::{Bencher, Criterion, ParameterizedBenchmark};

use bytecount::{
    count, naive_count, naive_count_32,
    num_chars, naive_num_chars,
};

fn random_bytes(len: usize) -> Vec<u8> {
    rand::thread_rng().gen_iter::<u8>().take(len).collect::<Vec<_>>()
}

static COUNTS : &[usize] = &[0, 10, 20, 30, 40, 50, 60, 70, 80, 90,
    100, 120, 140, 170, 210, 250, 300, 400, 500, 600, 700, 800, 900,
    1000, 1_000, 1_200, 1_400, 1_700, 2_100, 2_500, 3_000, 4_000,
    5_000, 6_000, 7_000, 8_000, 9_000, 10_000, 12_000, 14_000, 17_000,
    21_000, 25_000, 30_000, 100_000, 1_000_000];

fn get_counts() -> Vec<usize> {
    env::var("COUNTS").map(
            |s| s.split(',').map(
            |n| str::parse::<usize>(n).unwrap()).collect())
        .unwrap_or(COUNTS.to_owned())
}

fn get_config() -> Criterion {
    if env::var("CI").is_ok() {
        Criterion::default().nresamples(5_000)
                            .without_plots()
                            .measurement_time(Duration::new(2, 0))
                            .warm_up_time(Duration::new(1, 0))
    } else {
        Criterion::default()
    }
}

fn bench_counts(criterion: &mut Criterion) {
    fn naive(b: &mut Bencher, s: &usize) {
        let haystack =  random_bytes(*s);
        b.iter(|| naive_count(&haystack, 10))
    }
    fn naive_32(b: &mut Bencher, s: &usize) {
        let haystack =  random_bytes(*s);
        b.iter(|| naive_count_32(&haystack, 10))
    }
    fn hyper(b: &mut Bencher, s: &usize) {
        let haystack =  random_bytes(*s);
        b.iter(|| count(&haystack, 10))
    }
    let counts = get_counts();
    criterion.bench("counts",
        ParameterizedBenchmark::new("naive", naive, counts)
            .with_function("naive_32", naive_32)
            .with_function("hyper", hyper));
}

fn bench_num_chars(criterion: &mut Criterion) {
    fn naive(b: &mut Bencher, s: &usize) {
        let haystack =  random_bytes(*s);
        b.iter(|| naive_num_chars(&haystack))
    }
    fn hyper(b: &mut Bencher, s: &usize) {
        let haystack =  random_bytes(*s);
        b.iter(|| num_chars(&haystack))
    }
    let counts = get_counts();
    criterion.bench("num_chars",
        ParameterizedBenchmark::new("naive", naive, counts)
            .with_function("hyper", hyper));
}

criterion_group!(name = count_bench; config = get_config(); targets = bench_counts);
criterion_group!(name = num_chars_bench; config = get_config(); targets = bench_num_chars);
criterion_main!(count_bench, num_chars_bench);
