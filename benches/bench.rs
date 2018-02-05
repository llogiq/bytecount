#[macro_use]
extern crate criterion;
extern crate rand;
extern crate bytecount;

use std::borrow::Cow;
use std::env;
use rand::Rng;
use criterion::{Bencher, Criterion};

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


fn get_counts() -> Cow<'static, [usize]> {
    env::var("COUNTS").map(
            |s| Cow::Owned(s.split(',').map(
            |n| str::parse::<usize>(n).unwrap()).collect()))
        .unwrap_or(Cow::Borrowed(COUNTS))
}

fn bench_naive_count(criterion: &mut Criterion) {
    criterion.bench_function_over_inputs("naive_count",
        |b: &mut Bencher, s: &&usize| {
            let haystack =  random_bytes(**s);
            b.iter(|| naive_count(&haystack, 10))
        },
        get_counts().iter());
}

fn bench_naive_count_32(criterion: &mut Criterion) {
    criterion.bench_function_over_inputs("naive_count_32",
        |b: &mut Bencher, s: &&usize| {
            let haystack =  random_bytes(**s);
            b.iter(|| naive_count_32(&haystack, 10))
        },
        get_counts().iter());
}

fn bench_count(criterion: &mut Criterion) {
    criterion.bench_function_over_inputs("count",
        |b: &mut Bencher, s: &&usize| {
            let haystack =  random_bytes(**s);
            b.iter(|| count(&haystack, 10))
        },
        get_counts().iter());
}

fn bench_naive_num_chars(criterion: &mut Criterion) {
    criterion.bench_function_over_inputs("naive_num_chars",
        |b: &mut Bencher, s: &&usize| {
            let haystack =  random_bytes(**s);
            b.iter(|| naive_num_chars(&haystack))
        },
        get_counts().iter());
}

fn bench_num_chars(criterion: &mut Criterion) {
    criterion.bench_function_over_inputs("num_chars",
        |b: &mut Bencher, s: &&usize| {
            let haystack =  random_bytes(**s);
            b.iter(|| num_chars(&haystack))
        },
        get_counts().iter());
}

criterion_group!(count_bench, bench_naive_count, bench_naive_count_32,
                 bench_count);
criterion_group!(num_chars_bench, bench_naive_num_chars,
                 bench_num_chars);
criterion_main!(count_bench, num_chars_bench);
