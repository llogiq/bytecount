use std::{mem, simd::{u8x16, u64x2, u8x32, u8x64}, arch::x86_64};
use naive::{naive_count, naive_num_chars};
use simd::common::{SumU8s, BasicSum, chunk_count, chunk_num_chars};

unsafe fn sad_u8x16(lhs: u8x16, rhs: u8x16) -> u64x2 {
    mem::transmute(x86_64::_mm_sad_epu8(mem::transmute(lhs), mem::transmute(rhs)))
}

#[target_feature(enable = "sse2")]
unsafe fn sum_x64_sse2(u8s: &u8x64) -> usize {
    let mut store = [0; mem::size_of::<u8x64>()];
    u8s.store(&mut store, 0);

    let sums =
         sad_u8x16(u8x16::load(&store,  0), u8x16::splat(0)) +
         sad_u8x16(u8x16::load(&store, 16), u8x16::splat(0)) +
         sad_u8x16(u8x16::load(&store, 32), u8x16::splat(0)) +
         sad_u8x16(u8x16::load(&store, 48), u8x16::splat(0));

    (sums.extract(0) + sums.extract(1)) as usize
}

#[target_feature(enable = "sse2")]
unsafe fn sum_x32_sse2(u8s: &u8x32) -> usize {
    let mut store = [0; mem::size_of::<u8x32>()];
    u8s.store(&mut store, 0);

    let sums =
         sad_u8x16(u8x16::load(&store,  0), u8x16::splat(0)) +
         sad_u8x16(u8x16::load(&store, 16), u8x16::splat(0));

    (sums.extract(0) + sums.extract(1)) as usize
}

struct SadSum;
impl SumU8s for SadSum {
    fn sum_x64(u8s: &u8x64) -> usize {
        unsafe { sum_x64_sse2(u8s) }
    }
    fn sum_x32(u8s: &u8x32) -> usize {
        unsafe { sum_x32_sse2(u8s) }
    }
}

pub fn count(haystack: &[u8], needle: u8) -> usize {
    if haystack.len() < 32 {
        naive_count(haystack, needle)
    } else {
        if is_target_feature_detected!("sse2") {
            #[target_feature(enable = "sse2")]
            unsafe fn chunk_count_sadsum(haystack: &[u8], needle: u8) -> usize {
                chunk_count::<SadSum>(haystack, needle)
            }
            unsafe { chunk_count_sadsum(haystack, needle) }
        } else {
            chunk_count::<BasicSum>(haystack, needle)
        }
    }
}

pub fn num_chars(utf8_chars: &[u8]) -> usize {
    if utf8_chars.len() < 32 {
        naive_num_chars(utf8_chars)
    } else {
        if is_target_feature_detected!("sse2") {
            #[target_feature(enable = "sse2")]
            unsafe fn chunk_num_chars_sadsum(utf8_chars: &[u8]) -> usize {
                chunk_num_chars::<SadSum>(utf8_chars)
            }
            unsafe { chunk_num_chars_sadsum(utf8_chars) }
        } else {
            chunk_num_chars::<BasicSum>(utf8_chars)
        }
    }
}
