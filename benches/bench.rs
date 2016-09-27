#![feature(test)]

extern crate test;
extern crate rand;
extern crate bytecount;

use rand::Rng;

use bytecount::{count, naive_count};

fn random_bytes(len: usize) -> Vec<u8> {
    rand::thread_rng().gen_iter::<u8>().take(len).collect::<Vec<_>>()
}

macro_rules! bench {
    ($i: expr, $name_naive: ident, $name_hyper: ident) => {
        #[bench]
        fn $name_naive(b: &mut test::Bencher) {
            let haystack = random_bytes($i);
            b.iter(|| naive_count(&haystack, 10));
        }

        #[bench]
        fn $name_hyper(b: &mut test::Bencher) {
            let haystack = random_bytes($i);
            b.iter(|| count(&haystack, 10));
        }
    };
}

bench!(0, bench_0_naive, bench_0_hyper);
bench!(1, bench_1_naive, bench_1_hyper);
bench!(10, bench_10_naive, bench_10_hyper);
bench!(100, bench_100_naive, bench_100_hyper);
bench!(1000, bench_1000_naive, bench_1000_hyper);
bench!(10000, bench_10000_naive, bench_10000_hyper);
bench!(100000, bench_100000_naive, bench_100000_hyper);
bench!(1000000, bench_1000000_naive, bench_1000000_hyper);
