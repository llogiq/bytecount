#[macro_use]
extern crate bencher;
extern crate rand;
extern crate bytecount;

use rand::Rng;

use bytecount::{
    count, naive_count, naive_count_32,
    num_chars, naive_num_chars,
};

fn random_bytes(len: usize) -> Vec<u8> {
    rand::thread_rng().gen_iter::<u8>().take(len).collect::<Vec<_>>()
}

macro_rules! bench_count {
    ($i: expr, $name_naive: ident, $name_32: ident, $name_hyper: ident) => {
        fn $name_naive(b: &mut bencher::Bencher) {
            let haystack = random_bytes($i);
            b.iter(|| naive_count(&haystack, 10));
        }

        fn $name_32(b: &mut bencher::Bencher) {
            let haystack = random_bytes($i);
            b.iter(|| naive_count_32(&haystack, 10));
        }

        fn $name_hyper(b: &mut bencher::Bencher) {
            let haystack = random_bytes($i);
            b.iter(|| count(&haystack, 10));
        }
    };
}

macro_rules! bench_num_chars {
    ($i: expr, $name_naive: ident, $name_hyper: ident) => {
        fn $name_naive(b: &mut bencher::Bencher) {
            let haystack = random_bytes($i);
            b.iter(|| naive_num_chars(&haystack));
        }

        fn $name_hyper(b: &mut bencher::Bencher) {
            let haystack = random_bytes($i);
            b.iter(|| num_chars(&haystack));
        }
    };
}

bench_count!(0, bench_count_00000_naive, bench_count_00000_32, bench_count_00000_hyper);
bench_count!(10, bench_count_00010_naive, bench_count_00010_32, bench_count_00010_hyper);
bench_count!(20, bench_count_00020_naive, bench_count_00020_32, bench_count_00020_hyper);
bench_count!(30, bench_count_00030_naive, bench_count_00030_32, bench_count_00030_hyper);

bench_count!(40, bench_count_00040_naive, bench_count_00040_32, bench_count_00040_hyper);
bench_count!(50, bench_count_00050_naive, bench_count_00050_32, bench_count_00050_hyper);
bench_count!(60, bench_count_00060_naive, bench_count_00060_32, bench_count_00060_hyper);
bench_count!(70, bench_count_00070_naive, bench_count_00070_32, bench_count_00070_hyper);
bench_count!(80, bench_count_00080_naive, bench_count_00080_32, bench_count_00080_hyper);
bench_count!(90, bench_count_00090_naive, bench_count_00090_32, bench_count_00090_hyper);
bench_count!(100, bench_count_00100_naive, bench_count_00100_32, bench_count_00100_hyper);
bench_count!(120, bench_count_00120_naive, bench_count_00120_32, bench_count_00120_hyper);
bench_count!(140, bench_count_00140_naive, bench_count_00140_32, bench_count_00140_hyper);
bench_count!(170, bench_count_00170_naive, bench_count_00170_32, bench_count_00170_hyper);
bench_count!(210, bench_count_00210_naive, bench_count_00210_32, bench_count_00210_hyper);
bench_count!(250, bench_count_00250_naive, bench_count_00250_32, bench_count_00250_hyper);
bench_count!(300, bench_count_00300_naive, bench_count_00300_32, bench_count_00300_hyper);

bench_count!(400, bench_count_00400_naive, bench_count_00400_32, bench_count_00400_hyper);
bench_count!(500, bench_count_00500_naive, bench_count_00500_32, bench_count_00500_hyper);
bench_count!(600, bench_count_00600_naive, bench_count_00600_32, bench_count_00600_hyper);
bench_count!(700, bench_count_00700_naive, bench_count_00700_32, bench_count_00700_hyper);
bench_count!(800, bench_count_00800_naive, bench_count_00800_32, bench_count_00800_hyper);
bench_count!(900, bench_count_00900_naive, bench_count_00900_32, bench_count_00900_hyper);
bench_count!(1_000, bench_count_01000_naive, bench_count_01000_32, bench_count_01000_hyper);
bench_count!(1_200, bench_count_01200_naive, bench_count_01200_32, bench_count_01200_hyper);
bench_count!(1_400, bench_count_01400_naive, bench_count_01400_32, bench_count_01400_hyper);
bench_count!(1_700, bench_count_01700_naive, bench_count_01700_32, bench_count_01700_hyper);
bench_count!(2_100, bench_count_02100_naive, bench_count_02100_32, bench_count_02100_hyper);
bench_count!(2_500, bench_count_02500_naive, bench_count_02500_32, bench_count_02500_hyper);
bench_count!(3_000, bench_count_03000_naive, bench_count_03000_32, bench_count_03000_hyper);

bench_count!(4_000, bench_count_04000_naive, bench_count_04000_32, bench_count_04000_hyper);
bench_count!(5_000, bench_count_05000_naive, bench_count_05000_32, bench_count_05000_hyper);
bench_count!(6_000, bench_count_06000_naive, bench_count_06000_32, bench_count_06000_hyper);
bench_count!(7_000, bench_count_07000_naive, bench_count_07000_32, bench_count_07000_hyper);
bench_count!(8_000, bench_count_08000_naive, bench_count_08000_32, bench_count_08000_hyper);
bench_count!(9_000, bench_count_09000_naive, bench_count_09000_32, bench_count_09000_hyper);
bench_count!(10_000, bench_count_10000_naive, bench_count_10000_32, bench_count_10000_hyper);
bench_count!(12_000, bench_count_12000_naive, bench_count_12000_32, bench_count_12000_hyper);
bench_count!(14_000, bench_count_14000_naive, bench_count_14000_32, bench_count_14000_hyper);
bench_count!(17_000, bench_count_17000_naive, bench_count_17000_32, bench_count_17000_hyper);
bench_count!(21_000, bench_count_21000_naive, bench_count_21000_32, bench_count_21000_hyper);
bench_count!(25_000, bench_count_25000_naive, bench_count_25000_32, bench_count_25000_hyper);
bench_count!(30_000, bench_count_30000_naive, bench_count_30000_32, bench_count_30000_hyper);

bench_count!(100_000, bench_count_big_0100000_naive, bench_count_big_0100000_32, bench_count_big_0100000_hyper);
bench_count!(1_000_000, bench_count_big_1000000_naive, bench_count_big_1000000_32, bench_count_big_1000000_hyper);

benchmark_group!(bench_count_naive,
    bench_count_00000_naive, bench_count_00010_naive, bench_count_00020_naive,
    bench_count_00030_naive, bench_count_00040_naive, bench_count_00050_naive,
    bench_count_00060_naive, bench_count_00070_naive, bench_count_00080_naive,
    bench_count_00090_naive, bench_count_00100_naive, bench_count_00120_naive,
    bench_count_00140_naive, bench_count_00170_naive, bench_count_00210_naive,
    bench_count_00250_naive, bench_count_00300_naive, bench_count_00400_naive,
    bench_count_00500_naive, bench_count_00600_naive, bench_count_00700_naive,
    bench_count_00800_naive, bench_count_00900_naive, bench_count_01000_naive,
    bench_count_01200_naive, bench_count_01400_naive, bench_count_01700_naive,
    bench_count_02100_naive, bench_count_02500_naive, bench_count_03000_naive,
    bench_count_04000_naive, bench_count_05000_naive, bench_count_06000_naive,
    bench_count_07000_naive, bench_count_08000_naive, bench_count_09000_naive,
    bench_count_10000_naive, bench_count_12000_naive, bench_count_14000_naive,
    bench_count_17000_naive, bench_count_21000_naive, bench_count_25000_naive,
    bench_count_30000_naive, bench_count_big_0100000_naive, bench_count_big_1000000_naive);

benchmark_group!(bench_count_32,
    bench_count_00000_32, bench_count_00010_32, bench_count_00020_32,
    bench_count_00030_32, bench_count_00040_32, bench_count_00050_32,
    bench_count_00060_32, bench_count_00070_32, bench_count_00080_32,
    bench_count_00090_32, bench_count_00100_32, bench_count_00120_32,
    bench_count_00140_32, bench_count_00170_32, bench_count_00210_32,
    bench_count_00250_32, bench_count_00300_32, bench_count_00400_32,
    bench_count_00500_32, bench_count_00600_32, bench_count_00700_32,
    bench_count_00800_32, bench_count_00900_32, bench_count_01000_32,
    bench_count_01200_32, bench_count_01400_32, bench_count_01700_32,
    bench_count_02100_32, bench_count_02500_32, bench_count_03000_32,
    bench_count_04000_32, bench_count_05000_32, bench_count_06000_32,
    bench_count_07000_32, bench_count_08000_32, bench_count_09000_32,
    bench_count_10000_32, bench_count_12000_32, bench_count_14000_32,
    bench_count_17000_32, bench_count_21000_32, bench_count_25000_32,
    bench_count_30000_32, bench_count_big_0100000_32, bench_count_big_1000000_32);

benchmark_group!(bench_count_hyper,
    bench_count_00000_hyper, bench_count_00010_hyper, bench_count_00020_hyper,
    bench_count_00030_hyper, bench_count_00040_hyper, bench_count_00050_hyper,
    bench_count_00060_hyper, bench_count_00070_hyper, bench_count_00080_hyper,
    bench_count_00090_hyper, bench_count_00100_hyper, bench_count_00120_hyper,
    bench_count_00140_hyper, bench_count_00170_hyper, bench_count_00210_hyper,
    bench_count_00250_hyper, bench_count_00300_hyper, bench_count_00400_hyper,
    bench_count_00500_hyper, bench_count_00600_hyper, bench_count_00700_hyper,
    bench_count_00800_hyper, bench_count_00900_hyper, bench_count_01000_hyper,
    bench_count_01200_hyper, bench_count_01400_hyper, bench_count_01700_hyper,
    bench_count_02100_hyper, bench_count_02500_hyper, bench_count_03000_hyper,
    bench_count_04000_hyper, bench_count_05000_hyper, bench_count_06000_hyper,
    bench_count_07000_hyper, bench_count_08000_hyper, bench_count_09000_hyper,
    bench_count_10000_hyper, bench_count_12000_hyper, bench_count_14000_hyper,
    bench_count_17000_hyper, bench_count_21000_hyper, bench_count_25000_hyper,
    bench_count_30000_hyper, bench_count_big_0100000_hyper, bench_count_big_1000000_hyper);

bench_num_chars!(0, bench_num_chars_00000_naive, bench_num_chars_00000_hyper);
bench_num_chars!(10, bench_num_chars_00010_naive, bench_num_chars_00010_hyper);
bench_num_chars!(20, bench_num_chars_00020_naive, bench_num_chars_00020_hyper);
bench_num_chars!(30, bench_num_chars_00030_naive, bench_num_chars_00030_hyper);

bench_num_chars!(40, bench_num_chars_00040_naive, bench_num_chars_00040_hyper);
bench_num_chars!(50, bench_num_chars_00050_naive, bench_num_chars_00050_hyper);
bench_num_chars!(60, bench_num_chars_00060_naive, bench_num_chars_00060_hyper);
bench_num_chars!(70, bench_num_chars_00070_naive, bench_num_chars_00070_hyper);
bench_num_chars!(80, bench_num_chars_00080_naive, bench_num_chars_00080_hyper);
bench_num_chars!(90, bench_num_chars_00090_naive, bench_num_chars_00090_hyper);
bench_num_chars!(100, bench_num_chars_00100_naive, bench_num_chars_00100_hyper);
bench_num_chars!(120, bench_num_chars_00120_naive, bench_num_chars_00120_hyper);
bench_num_chars!(140, bench_num_chars_00140_naive, bench_num_chars_00140_hyper);
bench_num_chars!(170, bench_num_chars_00170_naive, bench_num_chars_00170_hyper);
bench_num_chars!(210, bench_num_chars_00210_naive, bench_num_chars_00210_hyper);
bench_num_chars!(250, bench_num_chars_00250_naive, bench_num_chars_00250_hyper);
bench_num_chars!(300, bench_num_chars_00300_naive, bench_num_chars_00300_hyper);

bench_num_chars!(400, bench_num_chars_00400_naive, bench_num_chars_00400_hyper);
bench_num_chars!(500, bench_num_chars_00500_naive, bench_num_chars_00500_hyper);
bench_num_chars!(600, bench_num_chars_00600_naive, bench_num_chars_00600_hyper);
bench_num_chars!(700, bench_num_chars_00700_naive, bench_num_chars_00700_hyper);
bench_num_chars!(800, bench_num_chars_00800_naive, bench_num_chars_00800_hyper);
bench_num_chars!(900, bench_num_chars_00900_naive, bench_num_chars_00900_hyper);
bench_num_chars!(1_000, bench_num_chars_01000_naive, bench_num_chars_01000_hyper);
bench_num_chars!(1_200, bench_num_chars_01200_naive, bench_num_chars_01200_hyper);
bench_num_chars!(1_400, bench_num_chars_01400_naive, bench_num_chars_01400_hyper);
bench_num_chars!(1_700, bench_num_chars_01700_naive, bench_num_chars_01700_hyper);
bench_num_chars!(2_100, bench_num_chars_02100_naive, bench_num_chars_02100_hyper);
bench_num_chars!(2_500, bench_num_chars_02500_naive, bench_num_chars_02500_hyper);
bench_num_chars!(3_000, bench_num_chars_03000_naive, bench_num_chars_03000_hyper);

bench_num_chars!(4_000, bench_num_chars_04000_naive, bench_num_chars_04000_hyper);
bench_num_chars!(5_000, bench_num_chars_05000_naive, bench_num_chars_05000_hyper);
bench_num_chars!(6_000, bench_num_chars_06000_naive, bench_num_chars_06000_hyper);
bench_num_chars!(7_000, bench_num_chars_07000_naive, bench_num_chars_07000_hyper);
bench_num_chars!(8_000, bench_num_chars_08000_naive, bench_num_chars_08000_hyper);
bench_num_chars!(9_000, bench_num_chars_09000_naive, bench_num_chars_09000_hyper);
bench_num_chars!(10_000, bench_num_chars_10000_naive, bench_num_chars_10000_hyper);
bench_num_chars!(12_000, bench_num_chars_12000_naive, bench_num_chars_12000_hyper);
bench_num_chars!(14_000, bench_num_chars_14000_naive, bench_num_chars_14000_hyper);
bench_num_chars!(17_000, bench_num_chars_17000_naive, bench_num_chars_17000_hyper);
bench_num_chars!(21_000, bench_num_chars_21000_naive, bench_num_chars_21000_hyper);
bench_num_chars!(25_000, bench_num_chars_25000_naive, bench_num_chars_25000_hyper);
bench_num_chars!(30_000, bench_num_chars_30000_naive, bench_num_chars_30000_hyper);

bench_num_chars!(100_000, bench_num_chars_big_0100000_naive, bench_num_chars_big_0100000_hyper);
bench_num_chars!(1_000_000, bench_num_chars_big_1000000_naive, bench_num_chars_big_1000000_hyper);

benchmark_group!(bench_num_chars_naive,
    bench_num_chars_00000_naive, bench_num_chars_00010_naive, bench_num_chars_00020_naive,
    bench_num_chars_00030_naive, bench_num_chars_00040_naive, bench_num_chars_00050_naive,
    bench_num_chars_00060_naive, bench_num_chars_00070_naive, bench_num_chars_00080_naive,
    bench_num_chars_00090_naive, bench_num_chars_00100_naive, bench_num_chars_00120_naive,
    bench_num_chars_00140_naive, bench_num_chars_00170_naive, bench_num_chars_00210_naive,
    bench_num_chars_00250_naive, bench_num_chars_00300_naive, bench_num_chars_00400_naive,
    bench_num_chars_00500_naive, bench_num_chars_00600_naive, bench_num_chars_00700_naive,
    bench_num_chars_00800_naive, bench_num_chars_00900_naive, bench_num_chars_01000_naive,
    bench_num_chars_01200_naive, bench_num_chars_01400_naive, bench_num_chars_01700_naive,
    bench_num_chars_02100_naive, bench_num_chars_02500_naive, bench_num_chars_03000_naive,
    bench_num_chars_04000_naive, bench_num_chars_05000_naive, bench_num_chars_06000_naive,
    bench_num_chars_07000_naive, bench_num_chars_08000_naive, bench_num_chars_09000_naive,
    bench_num_chars_10000_naive, bench_num_chars_12000_naive, bench_num_chars_14000_naive,
    bench_num_chars_17000_naive, bench_num_chars_21000_naive, bench_num_chars_25000_naive,
    bench_num_chars_30000_naive, bench_num_chars_big_0100000_naive, bench_num_chars_big_1000000_naive);

benchmark_group!(bench_num_chars_hyper,
    bench_num_chars_00000_hyper, bench_num_chars_00010_hyper, bench_num_chars_00020_hyper,
    bench_num_chars_00030_hyper, bench_num_chars_00040_hyper, bench_num_chars_00050_hyper,
    bench_num_chars_00060_hyper, bench_num_chars_00070_hyper, bench_num_chars_00080_hyper,
    bench_num_chars_00090_hyper, bench_num_chars_00100_hyper, bench_num_chars_00120_hyper,
    bench_num_chars_00140_hyper, bench_num_chars_00170_hyper, bench_num_chars_00210_hyper,
    bench_num_chars_00250_hyper, bench_num_chars_00300_hyper, bench_num_chars_00400_hyper,
    bench_num_chars_00500_hyper, bench_num_chars_00600_hyper, bench_num_chars_00700_hyper,
    bench_num_chars_00800_hyper, bench_num_chars_00900_hyper, bench_num_chars_01000_hyper,
    bench_num_chars_01200_hyper, bench_num_chars_01400_hyper, bench_num_chars_01700_hyper,
    bench_num_chars_02100_hyper, bench_num_chars_02500_hyper, bench_num_chars_03000_hyper,
    bench_num_chars_04000_hyper, bench_num_chars_05000_hyper, bench_num_chars_06000_hyper,
    bench_num_chars_07000_hyper, bench_num_chars_08000_hyper, bench_num_chars_09000_hyper,
    bench_num_chars_10000_hyper, bench_num_chars_12000_hyper, bench_num_chars_14000_hyper,
    bench_num_chars_17000_hyper, bench_num_chars_21000_hyper, bench_num_chars_25000_hyper,
    bench_num_chars_30000_hyper, bench_num_chars_big_0100000_hyper, bench_num_chars_big_1000000_hyper);

benchmark_main!(
    bench_count_naive, bench_count_32, bench_count_hyper,
    bench_num_chars_naive, bench_num_chars_hyper
);
