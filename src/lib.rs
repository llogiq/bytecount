//! Counting occurrences of a byte in a slice
//!
//! There are two versions, one naive and simple (`naive_count`) and one
//!  (`count`) that uses `unsafe` a lot, but is screamingly fast. The algorithm
//! is actually called "hyperscreamingcount".
//!
//! Usage is like you would expect (`count(haystack, needle)`).
#[cfg(target_pointer_width = "16")] const USIZE_BYTES: usize = 2;
#[cfg(target_pointer_width = "32")] const USIZE_BYTES: usize = 4;
#[cfg(target_pointer_width = "64")] const USIZE_BYTES: usize = 8;
const LO : usize = ::std::usize::MAX / 0xFF;
const HI : usize = LO * 128;
const EVERY_OTHER_BYTE_LO : usize = ::std::usize::MAX / 0xFFFF;
const EVERY_OTHER_BYTE : usize = EVERY_OTHER_BYTE_LO * 0xFF;

unsafe fn next(ptr: &mut *const usize) -> usize {
    let ret = **ptr;
    *ptr = ptr.offset(1);
    ret
}

unsafe fn next_4(ptr: &mut *const usize, needles: usize) -> [usize; 4] {
    let x = [next(ptr), next(ptr), next(ptr), next(ptr)];
    [mask_zero(x[0], needles), mask_zero(x[1], needles),
     mask_zero(x[2], needles), mask_zero(x[3], needles)]
}

fn mask_zero(x: usize, needles: usize) -> usize {
    let x = x ^ needles;
    !((((x & !HI) + !HI) | x) >> 7) & LO
}

fn reduce_counts(counts: usize) -> usize {
    let pair_sum = (counts & EVERY_OTHER_BYTE) + ((counts >> 8) & EVERY_OTHER_BYTE);
    pair_sum.wrapping_mul(EVERY_OTHER_BYTE_LO) >> ((USIZE_BYTES - 2) * 8)
}

fn arr_add(xs: [usize; 4], ys: [usize; 4]) -> [usize; 4] {
    [xs[0]+ys[0], xs[1]+ys[1], xs[2]+ys[2], xs[3]+ys[3]]
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
pub fn count(haystack: &[u8], needle: u8) -> usize {
    let len = haystack.len();
    if len < USIZE_BYTES * 2 { return naive_count(haystack, needle) }
    let needles = needle as usize * LO;
    unsafe {
        let mut ptr = haystack.as_ptr();
        let mut end = ptr.offset(len as isize);
        let mut count = 0;

        // Align start
        while (ptr as usize) & (USIZE_BYTES - 1) != 0 {
            if ptr == end {
                return count;
            }
            count += (*ptr == needle) as usize;
            ptr = ptr.offset(1);
        }

        // Align end
        while (end as usize) & (USIZE_BYTES - 1) != 0 {
            end = end.offset(-1);
            count += (*end == needle) as usize;
        }

        if ptr == end {
            return count;
        }

        // Read in aligned blocks
        let mut ptr = ptr as *const usize;
        let end = end as *const usize;

        // 8kB
        while ptr.offset(4 * 255) <= end {
            let mut counts = [0, 0, 0, 0];
            for _ in 0..255 {
                counts = arr_add(counts, next_4(&mut ptr, needles));
            }
            count += reduce_counts(counts[0]);
            count += reduce_counts(counts[1]);
            count += reduce_counts(counts[2]);
            count += reduce_counts(counts[3]);
        }

        // 1kB
        while ptr.offset(4 * 32) <= end {
            let mut counts = [0, 0, 0, 0];
            for _ in 0..32 {
                counts = arr_add(counts, next_4(&mut ptr, needles));
            }
            count += reduce_counts(counts[0] + counts[1] + counts[2] + counts[3]);
        }
        // 64B
        let mut counts = [0, 0, 0, 0];
        while ptr.offset(4 * 2) <= end {
            for _ in 0..2 {
                counts = arr_add(counts, next_4(&mut ptr, needles));
            }
        }
        count += reduce_counts(counts[0] + counts[1] + counts[2] + counts[3]);
        // 8B
        let mut counts = 0;
        while ptr < end {
            counts += mask_zero(next(&mut ptr), needles);
        }
        count + reduce_counts(counts)
    }
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
