//! count occurrences of a given byte, or the number of UTF-8 code points, in a
//! byte slice, fast.
//!
//! This crate has the [`count`](fn.count.html) method to count byte
//! occurrences (for example newlines) in a larger `&[u8]` slice.
//!
//! For example:
//!
//! ```rust
//! assert_eq!(5, bytecount::count(b"Hello, this is the bytecount crate!", b' '));
//! ```
//!
//! Also there is a [`num_chars`](fn.num_chars.html) method to count
//! the number of UTF8 characters in a slice. It will work the same as
//! `str::chars().count()` for byte slices of correct UTF-8 character
//! sequences. The result will likely be off for invalid sequences,
//! although the result is guaranteed to be between `0` and
//! `[_]::len()`, inclusive.
//!
//! Example:
//!
//! ```rust
//! let sequence = "Wenn ich ein Vöglein wär, flög ich zu Dir!";
//! assert_eq!(sequence.chars().count(),
//!            bytecount::num_chars(sequence.as_bytes()));
//! ```
//!
//! For completeness and easy comparison, the "naive" versions of both
//! count and num_chars are provided. Those are also faster if used on
//! predominantly small strings. The
//! [`naive_count_32`](fn.naive_count_32.html) method can be faster
//! still on small strings.

#![deny(missing_docs)]

#![cfg_attr(feature = "simd-accel", feature(stdsimd))]

#![cfg_attr(not(feature = "target_feature"), no_std)]
#![cfg_attr(feature = "target_feature", feature(cfg_target_feature, target_feature))]

mod naive;
pub use naive::*;

#[cfg(not(feature = "simd-accel"))]
mod integer_simd;
#[cfg(not(feature = "simd-accel"))]
pub use integer_simd::*;

#[cfg(feature = "simd-accel")]
mod simd;
#[cfg(feature = "simd-accel")]
pub use simd::*;
