use naive::{naive_count, naive_num_chars};
use core::{mem, ptr, usize};

fn splat(byte: u8) -> usize {
    let lo = usize::MAX / 0xFF;
    lo * byte as usize
}

unsafe fn usize_load_unchecked(bytes: &[u8], offset: usize) -> usize {
    let mut output = 0;
    ptr::copy_nonoverlapping(
        bytes.as_ptr().offset(offset as isize),
        &mut output as *mut usize as *mut u8,
        mem::size_of::<usize>()
    );
    output
}

fn bytewise_equal(lhs: usize, rhs: usize) -> usize {
    let lo = usize::MAX / 0xFF;
    let hi = lo << 7;

    let x = lhs ^ rhs;
    !((((x & !hi) + !hi) | x) >> 7) & lo
}

fn sum_usize(values: usize) -> usize {
    let every_other_byte_lo = usize::MAX / 0xFFFF;
    let every_other_byte = every_other_byte_lo * 0xFF;

    // Pairwise reduction to avoid overflow on next step.
    let pair_sum: usize = (values & every_other_byte) + ((values >> 8) & every_other_byte);

    // Multiplication results in top two bytes holding sum.
    pair_sum.wrapping_mul(every_other_byte_lo) >> ((mem::size_of::<usize>() - 2) * 8)
}

fn is_leading_utf8_byte(values: usize) -> usize {
    // a leading UTF-8 byte is one which does not start with the bits 10.
    ((!values >> 7) | (values >> 6)) & splat(1)
}

fn chunk_count(haystack: &[u8], needle: u8) -> usize {
    let chunksize = mem::size_of::<usize>();
    assert!(haystack.len() >= chunksize);

    unsafe {
        let mut offset = 0;
        let mut count = 0;

        let needles = splat(needle);

        // 2040
        while haystack.len() >= offset + chunksize * 255 {
            let mut counts = 0;
            for _ in 0..255 {
                counts += bytewise_equal(usize_load_unchecked(haystack, offset), needles);
                offset += chunksize;
            }
            count += sum_usize(counts);
        }

        // 8
        let mut counts = 0;
        for i in 0..(haystack.len() - offset) / chunksize {
            counts += bytewise_equal(usize_load_unchecked(haystack, offset + i * chunksize), needles);
        }
        if haystack.len() % 8 != 0 {
            let mask = !(!0 >> ((haystack.len() % chunksize) * 8));
            counts += bytewise_equal(usize_load_unchecked(haystack, haystack.len() - chunksize), needles) & mask;
        }
        count += sum_usize(counts);

        count
    }
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
    if haystack.len() < 8 {
        naive_count(haystack, needle)
    } else {
        chunk_count(haystack, needle)
    }
}

fn chunk_num_chars(utf8_chars: &[u8]) -> usize {
    let chunksize = mem::size_of::<usize>();
    assert!(utf8_chars.len() >= chunksize);

    unsafe {
        let mut offset = 0;
        let mut count = 0;

        // 2040
        while utf8_chars.len() >= offset + chunksize * 255 {
            let mut counts = 0;
            for _ in 0..255 {
                counts += is_leading_utf8_byte(usize_load_unchecked(utf8_chars, offset));
                offset += chunksize;
            }
            count += sum_usize(counts);
        }

        // 8
        let mut counts = 0;
        for i in 0..(utf8_chars.len() - offset) / chunksize {
            counts += is_leading_utf8_byte(usize_load_unchecked(utf8_chars, offset + i * chunksize));
        }
        if utf8_chars.len() % 8 != 0 {
            let mask = !(!0 >> ((utf8_chars.len() % chunksize) * 8));
            counts += is_leading_utf8_byte(usize_load_unchecked(utf8_chars, utf8_chars.len() - chunksize)) & mask;
        }
        count += sum_usize(counts);

        count
    }
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
/// let char_count = bytecount::num_chars(swordfish.as_bytes());
/// assert_eq!(char_count, 4);
/// ```
pub fn num_chars(utf8_chars: &[u8]) -> usize {
    if utf8_chars.len() < 8 {
        naive_num_chars(utf8_chars)
    } else {
        chunk_num_chars(utf8_chars)
    }
}
