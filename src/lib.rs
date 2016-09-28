//! Counting occurrences of a byte in a slice
#[cfg(feature = "simd-accel")]
extern crate simd;

use std::{cmp, mem, slice};

#[cfg(feature = "simd-accel")]
use simd::u8x16;
#[cfg(feature = "avx-accel")]
use simd::x86::sse2::Sse2U8x16;
#[cfg(feature = "avx-accel")]
use simd::x86::avx::{LowHigh128, u8x32};


trait ByteChunk: Copy {
    fn splat(byte: u8) -> Self;
    fn bytewise_equal(self, other: Self) -> Self;
    fn increment(self, incr: Self) -> Self;
    fn sum(self) -> usize;
}

impl ByteChunk for usize {
    fn splat(byte: u8) -> Self {
        let lo = std::usize::MAX / 0xFF;
        lo * byte as usize
    }

    fn bytewise_equal(self, other: Self) -> Self {
        let lo = std::usize::MAX / 0xFF;
        let hi = lo << 7;

        let x = self ^ other;
        !((((x & !hi) + !hi) | x) >> 7) & lo
    }

    fn increment(self, incr: Self) -> Self {
        self + incr
    }

    fn sum(self) -> usize {
        let every_other_byte_lo = std::usize::MAX / 0xFFFF;
        let every_other_byte = every_other_byte_lo * 0xFF;

        // Pairwise reduction to avoid overflow on next step.
        let pair_sum = (self & every_other_byte) + ((self >> 8) & every_other_byte);

        // Multiplication results in top two bytes holding sum.
        pair_sum.wrapping_mul(every_other_byte_lo) >> ((mem::size_of::<usize>() - 2) * 8)
    }
}

#[cfg(feature = "simd-accel")]
impl ByteChunk for u8x16 {
    fn splat(byte: u8) -> Self {
        Self::splat(byte)
    }

    fn bytewise_equal(self, other: Self) -> Self {
        self.eq(other).to_repr().to_u8()
    }

    fn increment(self, incr: Self) -> Self {
        // incr on -1
        self - incr
    }

    fn sum(self) -> usize {
        let mut count = 0;
        for i in 0..16 {
            count += self.extract(i) as usize;
        }
        count
    }
}

#[cfg(feature = "avx-accel")]
impl ByteChunk for u8x32 {
    fn splat(byte: u8) -> Self {
        Self::splat(byte)
    }

    fn bytewise_equal(self, other: Self) -> Self {
        self.eq(other).to_repr().to_u8()
    }

    fn increment(self, incr: Self) -> Self {
        // incr on -1
        self - incr
    }

    fn sum(self) -> usize {
        let zero = u8x16::splat(0);
        let sad_lo = self.low().sad(zero);
        let sad_hi = self.high().sad(zero);

        let mut count = 0;
        count += (sad_lo.extract(0) + sad_lo.extract(1)) as usize;
        count += (sad_hi.extract(0) + sad_hi.extract(1)) as usize;
        count
    }
}


fn chunk_align<Chunk: ByteChunk>(x: &[u8]) -> (&[u8], &[[Chunk; 4]], &[u8]) {
    let align = mem::size_of::<[Chunk; 4]>();

    let offset_ptr = (x.as_ptr() as usize) % align;
    let offset_end = (x.as_ptr() as usize + x.len()) % align;

    let d2 = x.len().saturating_sub(offset_end);
    let d1 = cmp::min((align - offset_ptr) % align, d2);

    let mid = &x[d1..d2];
    assert!(mid.len() % align == 0);
    let mid = unsafe {
        slice::from_raw_parts(mid.as_ptr() as *const [Chunk; 4], mid.len() / align)
    };

    (&x[..d1], mid, &x[d2..])
}

fn arr_byte_equal<Chunk: ByteChunk>(mut xs: [Chunk; 4], needles: Chunk) -> [Chunk; 4] {
    for i in 0..4 {
        xs[i] = xs[i].bytewise_equal(needles);
    }
    xs
}

fn arr_incr<Chunk: ByteChunk>(xs: [Chunk; 4], ys: [Chunk; 4]) -> [Chunk; 4] {
    [
        xs[0].increment(ys[0]),
        xs[1].increment(ys[1]),
        xs[2].increment(ys[2]),
        xs[3].increment(ys[3])
    ]
}

fn chunk_count<Chunk: ByteChunk>(haystack: &[[Chunk; 4]], needle: u8) -> usize {
    let zero = Chunk::splat(0);
    let needles = Chunk::splat(needle);
    let mut count = 0;
    let mut i = 0;

    while i < haystack.len() {
        let mut counts = [zero, zero, zero, zero];

        let end = cmp::min(i + 255, haystack.len());
        for &c in &haystack[i..end] {
            counts = arr_incr(counts, arr_byte_equal(c, needles));
        }
        i = end;

        for i in 0..4 {
            count += counts[i].sum();
        }
    }

    count
}

fn count_generic<Chunk: ByteChunk>(haystack: &[u8], needle: u8) -> usize {
    let (pre, mid, post) = chunk_align::<Chunk>(haystack);
    naive_count(pre, needle) + chunk_count(mid, needle) + naive_count(post, needle)
}


/// Count occurrences of a byte in a slice of bytes, fast
///
/// # Examples
///
/// ```
/// let s = b"This is a Text with spaces";
/// let number_of_spaces = bytecount::count(s, b' ');
/// assert_eq!(number_of_spaces, 5);
/// ```
#[cfg(not(feature = "simd-accel"))]
pub fn count(haystack: &[u8], needle: u8) -> usize {
    count_generic::<usize>(haystack, needle)
}

#[cfg(all(feature = "simd-accel", not(feature = "avx-accel")))]
pub fn count(haystack: &[u8], needle: u8) -> usize {
    count_generic::<u8x16>(haystack, needle)
}

#[cfg(feature = "avx-accel")]
pub fn count(haystack: &[u8], needle: u8) -> usize {
    count_generic::<u8x32>(haystack, needle)
}

/// Count occurrences of a byte in a slice of bytes, simple
///
/// # Examples
///
/// ```
/// let s = b"This is yet another Text with spaces";
/// let number_of_spaces = bytecount::naive_count(s, b' ');
/// assert_eq!(number_of_spaces, 6);
/// ```
pub fn naive_count(haystack: &[u8], needle: u8) -> usize {
    haystack.iter().filter(|&&c| c == needle).count()
}
