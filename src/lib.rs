//! Counting occurrences of a byte in a slice
//!
//! There are two versions, one naive and simple (`naive_count`) and one
//!  (`count`) that uses `unsafe`, but is hyperscreamingly fast.
//!
//! Usage is like you would expect (`count(haystack, needle)`).
use std::{cmp, mem, slice};

const LO : usize = ::std::usize::MAX / 0xFF;
const HI : usize = LO * 128;
const EVERY_OTHER_BYTE_LO : usize = ::std::usize::MAX / 0xFFFF;
const EVERY_OTHER_BYTE : usize = EVERY_OTHER_BYTE_LO * 0xFF;

/// Count occurrences of a byte in a slice of bytes, fast
///
/// # Examples
///
/// ```
/// let s = b"This is a Text with spaces";
/// let number_of_spaces = bytecount::count(s, b' ');
/// assert_eq!(number_of_spaces, 5);
/// ```
pub fn count(haystack: &[u8], needle: u8) -> usize {
    let (pre, mid, post) = bytes_to_usizes(haystack);
    naive_count(pre, needle) + usize_count(mid, needle) + naive_count(post, needle)
}

fn bytes_to_usizes(x: &[u8]) -> (&[u8], &[[usize; 4]], &[u8]) {
    let align = mem::size_of::<[usize; 4]>();

    let offset_ptr = (x.as_ptr() as usize) % align;
    let offset_end = (x.as_ptr() as usize + x.len()) % align;

    let d2 = x.len().saturating_sub(offset_end);
    let d1 = cmp::min((align - offset_ptr) % align, d2);

    let mid = &x[d1..d2];
    assert!(mid.len() % align == 0);
    let mid = unsafe {
        slice::from_raw_parts(mid.as_ptr() as *const [usize; 4], mid.len() / align)
    };

    (&x[..d1], mid, &x[d2..])
}


fn vector_not(x: usize) -> usize {
    !((((x & !HI) + !HI) | x) >> 7) & LO
}

fn arr_byte_equal(mut xs: [usize; 4], needles: usize) -> [usize; 4] {
    for i in 0..4 {
        xs[i] = vector_not(xs[i] ^ needles);
    }
    xs
}

fn arr_add(xs: [usize; 4], ys: [usize; 4]) -> [usize; 4] {
    [xs[0] + ys[0], xs[1] + ys[1], xs[2] + ys[2], xs[3] + ys[3]]
}


fn sum_bytes(counts: usize) -> usize {
    // Pairwise reduction to avoid overflow on next step.
    let pair_sum = (counts & EVERY_OTHER_BYTE) + ((counts >> 8) & EVERY_OTHER_BYTE);

    // Multiplication results in top two bytes holding sum.
    pair_sum.wrapping_mul(EVERY_OTHER_BYTE_LO) >> ((mem::size_of::<usize>() - 2) * 8)
}


fn usize_count(haystack: &[[usize; 4]], needle: u8) -> usize {
    let needles = needle as usize * LO;
    let mut count = 0;
    let mut i = 0;

    while i < haystack.len() {
        let mut counts = [0, 0, 0, 0];

        let end = cmp::min(i + 255, haystack.len());
        for &c in &haystack[i..end] {
            counts = arr_add(counts, arr_byte_equal(c, needles));
        }
        i = end;

        for &c in &counts {
            count += sum_bytes(c);
        }
    }

    count
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
