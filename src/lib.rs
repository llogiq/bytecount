//! Counting occurrences of a byte in a slice
#[cfg(feature = "simd-accel")]
extern crate simd;

use std::{cmp, mem, ops, slice, usize};

#[cfg(feature = "simd-accel")]
use simd::u8x16;
#[cfg(feature = "avx-accel")]
use simd::x86::sse2::Sse2U8x16;
#[cfg(feature = "avx-accel")]
use simd::x86::avx::{LowHigh128, u8x32};


trait ByteChunk: Copy {
    type Splat: Copy;
    fn splat(byte: u8) -> Self::Splat;
    fn from_splat(splat: Self::Splat) -> Self;
    fn bytewise_equal(self, other: Self::Splat) -> Self;
    fn increment(self, incr: Self) -> Self;
    fn sum(&self) -> usize;
}

impl ByteChunk for usize {
    type Splat = Self;

    fn splat(byte: u8) -> Self {
        let lo = usize::MAX / 0xFF;
        lo * byte as usize
    }

    fn from_splat(splat: Self) -> Self {
        splat
    }

    fn bytewise_equal(self, other: Self) -> Self {
        let lo = usize::MAX / 0xFF;
        let hi = lo << 7;

        let x = self ^ other;
        !((((x & !hi) + !hi) | x) >> 7)