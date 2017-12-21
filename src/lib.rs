//! Counting occurrences of a byte in a slice

#![no_std]

#[cfg(feature = "simd-accel")]
extern crate simd;

use core::{cmp, mem, ops, slice, usize};

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
    fn mask(self, other: Self::Splat) -> Self;
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

    fn mask(self, other: Self) -> Self {
        self & other
    }

    fn bytewise_equal(self, other: Self) -> Self {
        let lo = usize::MAX / 0xFF;
        let hi = lo << 7;

        let x = self ^ other;
        !((((x & !hi) + !hi) | x) >> 7) & lo
    }

    fn increment(self, incr: Self) -> Self {
        self + incr
    }

    fn sum(&self) -> usize {
        let every_other_byte_lo = usize::MAX / 0xFFFF;
        let every_other_byte = every_other_byte_lo * 0xFF;

        // Pairwise reduction to avoid overflow on next step.
        let pair_sum: usize = (self & every_other_byte) + ((self >> 8) & every_other_byte);

        // Multiplication results in top two bytes holding sum.
        pair_sum.wrapping_mul(every_other_byte_lo) >> ((mem::size_of::<usize>() - 2) * 8)
    }
}

#[cfg(feature = "simd-accel")]
impl ByteChunk for u8x16 {
    type Splat = Self;

    fn splat(byte: u8) -> Self {
        Self::splat(byte)
    }

    fn from_splat(splat: Self) -> Self {
        splat
    }

    fn mask(self, other: Self) -> Self {
        self & other
    }

    fn bytewise_equal(self, other: Self) -> Self {
        self.eq(other).to_repr().to_u8()
    }

    fn increment(self, incr: Self) -> Self {
        // incr on -1
        self - incr
    }

    fn sum(&self) -> usize {
        let mut count = 0;
        for i in 0..16 {
            count += self.extract(i) as usize;
        }
        count
    }
}

#[cfg(feature = "avx-accel")]
impl ByteChunk for u8x32 {
    type Splat = Self;

    fn splat(byte: u8) -> Self {
        Self::splat(byte)
    }

    fn from_splat(splat: Self) -> Self {
        splat
    }

    fn mask(self, other: Self) -> Self {
        self & other
    }

    fn bytewise_equal(self, other: Self) -> Self {
        self.eq(other).to_repr().to_u8()
    }

    fn increment(self, incr: Self) -> Self {
        // incr on -1
        self - incr
    }

    fn sum(&self) -> usize {
        let zero = u8x16::splat(0);
        let sad_lo = self.low().sad(zero);
        let sad_hi = self.high().sad(zero);

        let mut count = 0;
        count += (sad_lo.extract(0) + sad_lo.extract(1)) as usize;
        count += (sad_hi.extract(0) + sad_hi.extract(1)) as usize;
        count
    }
}

impl<T> ByteChunk for [T; 4]
    where T: ByteChunk<Splat = T>
{
    type Splat = T;

    fn splat(byte: u8) -> T {
        T::splat(byte)
    }

    fn from_splat(splat: T) -> Self {
        [splat, splat, splat, splat]
    }

    fn mask(mut self, other: Self::Splat) -> Self {
        for t in self[..].iter_mut() {
            *t = t.mask(other);
        }
        self
    }

    fn bytewise_equal(mut self, needles: Self::Splat) -> Self {
        for t in self[..].iter_mut() {
            *t = t.bytewise_equal(needles);
        }
        self
    }

    fn increment(self, incr: Self) -> Self {
        [self[0].increment(incr[0]),
         self[1].increment(incr[1]),
         self[2].increment(incr[2]),
         self[3].increment(incr[3])]
    }

    fn sum(&self) -> usize {
        self[..].iter().map(ByteChunk::sum).fold(0, ops::Add::add)
    }
}


fn chunk_align<Chunk: ByteChunk>(x: &[u8]) -> (&[u8], &[Chunk], &[u8]) {
    let align = mem::size_of::<Chunk>();

    let offset_ptr = (x.as_ptr() as usize) % align;
    let offset_end = (x.as_ptr() as usize + x.len()) % align;

    let d2 = x.len().saturating_sub(offset_end);
    let d1 = cmp::min((align - offset_ptr) % align, d2);

    let (init, tail) = x.split_at(d2);
    let (init, mid) = init.split_at(d1);
    assert_eq!(mid.len() % align, 0);
    let mid = unsafe { slice::from_raw_parts(mid.as_ptr() as *const Chunk, mid.len() / align) };

    (init, mid, tail)
}

fn chunk_count<Chunk: ByteChunk>(haystack: &[Chunk], needle: u8) -> usize {
    let zero = Chunk::splat(0);
    let needles = Chunk::splat(needle);
    let mut count = 0;
    let mut i = 0;

    while i < haystack.len() {
        let mut counts = Chunk::from_splat(zero);

        let end = cmp::min(i + 255, haystack.len());
        for &chunk in &haystack[i..end] {
            counts = counts.increment(chunk.bytewise_equal(needles));
        }
        i = end;

        count += counts.sum();
    }

    count
}

fn count_generic<Chunk: ByteChunk<Splat = Chunk>>(naive_below: usize,
                                                  group_above: usize,
                                                  haystack: &[u8],
                                                  needle: u8)
                                                  -> usize {
    let mut count = 0;

    // Extract pre/post so naive_count is only inlined once.
    let len = haystack.len();
    let unchunked = if len < naive_below {
        [haystack, &haystack[0..0]]
    } else if len > group_above {
        let (pre, mid, post) = chunk_align::<[Chunk; 4]>(haystack);
        count += chunk_count(mid, needle);
        [pre, post]
    } else {
        let (pre, mid, post) = chunk_align::<Chunk>(haystack);
        count += chunk_count(mid, needle);
        [pre, post]
    };

    for &slice in &unchunked {
        count += naive_count(slice, needle);
    }

    count
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
    // Never use [usize; 4]
    count_generic::<usize>(32, usize::MAX, haystack, needle)
}

#[cfg(all(feature = "simd-accel", not(feature = "avx-accel")))]
pub fn count(haystack: &[u8], needle: u8) -> usize {
    count_generic::<u8x16>(32, 4096, haystack, needle)
}

#[cfg(feature = "avx-accel")]
pub fn count(haystack: &[u8], needle: u8) -> usize {
    count_generic::<u8x32>(64, 4096, haystack, needle)
}

/// Count up to `(2^32)-1` occurrences of a byte in a slice
/// of bytes, simple
///
/// # Example
///
/// ```
/// let s = b"This is yet another Text with spaces";
/// let number_of_spaces = bytecount::naive_count(s, b' ');
/// assert_eq!(number_of_spaces, 6);
/// ```
pub fn naive_count_32(haystack: &[u8], needle: u8) -> usize {
    haystack.iter().fold(0, |n, c| n + (*c == needle) as u32) as usize
}

/// Count occurrences of a byte in a slice of bytes, simple
///
/// # Example
///
/// ```
/// let s = b"This is yet another Text with spaces";
/// let number_of_spaces = bytecount::naive_count(s, b' ');
/// assert_eq!(number_of_spaces, 6);
/// ```
pub fn naive_count(haystack: &[u8], needle: u8) -> usize {
    haystack.iter().fold(0, |n, c| n + (*c == needle) as usize)
}


fn chunk_num_chars<Chunk: ByteChunk>(haystack: &[Chunk]) -> usize {
    let zero = Chunk::splat(0);
    let needles = Chunk::splat(0b10_000000);
    let mask = Chunk::splat(0b11_000000);
    let mut count = 0;
    let mut i = 0;

    while i < haystack.len() {
        let mut counts = Chunk::from_splat(zero);

        let end = cmp::min(i + 255, haystack.len());
        for &chunk in &haystack[i..end] {
            counts = counts.increment(chunk.mask(mask).bytewise_equal(needles));
        }
        i = end;

        count += counts.sum();
    }

    count
}

fn num_chars_generic<Chunk: ByteChunk<Splat = Chunk>>(naive_below: usize,
                                                    group_above: usize,
                                                    haystack: &[u8])
                                                    -> usize {
    // Extract pre/post so naive_count is only inlined once.
    let len = haystack.len();
    let mut count = len;
    let unchunked = if len < naive_below {
        [haystack, &haystack[0..0]]
    } else if len > group_above {
        let (pre, mid, post) = chunk_align::<[Chunk; 4]>(haystack);
        count -= chunk_num_chars(mid);
        [pre, post]
    } else {
        let (pre, mid, post) = chunk_align::<Chunk>(haystack);
        count -= chunk_num_chars(mid);
        [pre, post]
    };

    for &slice in &unchunked {
        count -= slice.len();
        count += naive_num_chars(slice);
    }

    count
}


/// Count the number of UTF-8 encoded unicode codepoints in a slice of bytes, fast
///
/// This function is safe to use on any byte array, valid UTF-8 or not,
/// but the output is only meaningful for well-formed UTF-8.
///
/// # Example
///
/// ```
/// let swordfish = "メカジキ";
/// let char_count = bytecount::naive_num_chars(swordfish.as_bytes());
/// assert_eq!(char_count, 4);
/// ```
#[cfg(not(feature = "simd-accel"))]
pub fn num_chars(haystack: &[u8]) -> usize {
    // Never use [usize; 4]
    num_chars_generic::<usize>(32, usize::MAX, haystack)
}

#[cfg(all(feature = "simd-accel", not(feature = "avx-accel")))]
pub fn num_chars(haystack: &[u8]) -> usize {
    num_chars_generic::<u8x16>(32, 4096, haystack)
}

#[cfg(feature = "avx-accel")]
pub fn num_chars(haystack: &[u8]) -> usize {
    num_chars_generic::<u8x32>(64, 4096, haystack)
}

/// Count the number of UTF-8 encoded unicode codepoints in a slice of bytes, simple
///
/// This function is safe to use on any byte array, valid UTF-8 or not,
/// but the output is only meaningful for well-formed UTF-8.
///
/// # Example
///
/// ```
/// let swordfish = "メカジキ";
/// let char_count = bytecount::naive_num_chars(swordfish.as_bytes());
/// assert_eq!(char_count, 4);
/// ```
pub fn naive_num_chars(haystack: &[u8]) -> usize {
    haystack.iter().filter(|&&byte| (byte >> 6) != 0b10).count()
}

