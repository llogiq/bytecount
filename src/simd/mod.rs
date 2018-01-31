mod common;

// Modern x86 machines can sum a vector of bytes very quickly.
// Runtime feature detection is not available with no_std.
#[cfg(all(feature = "target_feature", any(target_arch = "x86", target_arch = "x86_64")))]
mod x86;
#[cfg(all(feature = "target_feature", any(target_arch = "x86", target_arch = "x86_64")))]
use self::x86 as platform;

#[cfg(not(all(feature = "target_feature", any(target_arch = "x86", target_arch = "x86_64"))))]
mod platform_independent {
    use naive::{naive_count, naive_num_chars};
    use super::common::{BasicSum, chunk_count, chunk_num_chars};

    pub fn count(haystack: &[u8], needle: u8) -> usize {
        if haystack.len() < 32 {
            naive_count(haystack, needle)
        } else {
            chunk_count::<BasicSum>(haystack, needle)
        }
    }

    pub fn num_chars(utf8_chars: &[u8]) -> usize {
        if utf8_chars.len() < 32 {
            naive_num_chars(utf8_chars)
        } else {
            chunk_num_chars::<BasicSum>(utf8_chars)
        }
    }
}
#[cfg(not(all(feature = "target_feature", any(target_arch = "x86", target_arch = "x86_64"))))]
use self::platform_independent as platform;

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
    platform::count(haystack, needle)
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
    platform::num_chars(utf8_chars)
}
