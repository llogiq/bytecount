#[cfg(feature = "target_feature")]
use std::{mem, simd::{u8x32, u8x64}};
#[cfg(not(feature = "target_feature"))]
use core::{mem, simd::{u8x32, u8x64}};

pub trait SumU8s {
    fn sum_x64(u8s: &u8x64) -> usize;
    fn sum_x32(u8s: &u8x32) -> usize;
}

pub struct BasicSum;
impl SumU8s for BasicSum {
    fn sum_x64(u8s: &u8x64) -> usize {
        let mut store = [0; mem::size_of::<u8x64>()];
        u8s.store(&mut store, 0);
        store.iter().map(|&e| e as usize).sum()
    }
    fn sum_x32(u8s: &u8x32) -> usize {
        let mut store = [0; mem::size_of::<u8x32>()];
        u8s.store(&mut store, 0);
        store.iter().map(|&e| e as usize).sum()
    }
}

const MASK: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

pub fn chunk_count<Summer: SumU8s>(haystack: &[u8], needle: u8) -> usize {
    assert!(haystack.len() >= 32);

    unsafe {
        let mut offset = 0;
        let mut count = 0;

        let needles_x64 = u8x64::splat(needle);

        // 16320
        while haystack.len() >= offset + 64 * 255 {
            let mut counts = u8x64::splat(0);;
            for _ in 0..255 {
                counts -= u8x64::load_unchecked(haystack, offset).eq(needles_x64).as_u8x64();
                offset += 64;
            }
            count += Summer::sum_x64(&counts);
        }

        // 2048
        if haystack.len() >= offset + 64 * 128 {
            let mut counts = u8x64::splat(0);;
            for _ in 0..128 {
                counts -= u8x64::load_unchecked(haystack, offset).eq(needles_x64).as_u8x64();
                offset += 64;
            }
            count += Summer::sum_x64(&counts);
        }

        let needles_x32 = u8x32::splat(needle);

        // 32
        let mut counts = u8x32::splat(0);
        for i in 0..(haystack.len() - offset) / 32 {
            counts -= u8x32::load_unchecked(haystack, offset + i * 32).eq(needles_x32).as_u8x32();
        }
        if haystack.len() % 32 != 0 {
            counts -= u8x32::load_unchecked(haystack, haystack.len() - 32).eq(needles_x32).as_u8x32() &
                      u8x32::load_unchecked(&MASK, haystack.len() % 32);
        }
        count += Summer::sum_x32(&counts);

        count
    }
}

fn is_leading_utf8_byte_x64(u8s: u8x64) -> u8x64 {
    (u8s & u8x64::splat(0b1100_0000)).ne(u8x64::splat(0b1000_0000)).as_u8x64()
}

fn is_leading_utf8_byte_x32(u8s: u8x32) -> u8x32 {
    (u8s & u8x32::splat(0b1100_0000)).ne(u8x32::splat(0b1000_0000)).as_u8x32()
}

pub fn chunk_num_chars<Summer: SumU8s>(utf8_chars: &[u8]) -> usize {
    assert!(utf8_chars.len() >= 32);

    unsafe {
        let mut offset = 0;
        let mut count = 0;

        // 16320
        while utf8_chars.len() >= offset + 64 * 255 {
            let mut counts = u8x64::splat(0);;
            for _ in 0..255 {
                counts -= is_leading_utf8_byte_x64(u8x64::load_unchecked(utf8_chars, offset));
                offset += 64;
            }
            count += Summer::sum_x64(&counts);
        }

        // 2048
        if utf8_chars.len() >= offset + 64 * 128 {
            let mut counts = u8x64::splat(0);;
            for _ in 0..128 {
                counts -= is_leading_utf8_byte_x64(u8x64::load_unchecked(utf8_chars, offset));
                offset += 64;
            }
            count += Summer::sum_x64(&counts);
        }

        // 32
        let mut counts = u8x32::splat(0);
        for i in 0..(utf8_chars.len() - offset) / 32 {
            counts -= is_leading_utf8_byte_x32(u8x32::load_unchecked(utf8_chars, offset + i * 32));
        }
        if utf8_chars.len() % 32 != 0 {
            counts -= is_leading_utf8_byte_x32(u8x32::load_unchecked(utf8_chars, utf8_chars.len() - 32)) &
                      u8x32::load_unchecked(&MASK, utf8_chars.len() % 32);
        }
        count += Summer::sum_x32(&counts);

        count
    }
}
