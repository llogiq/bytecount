extern crate bytecount;
#[macro_use] extern crate quickcheck;
extern crate rand;

use bytecount::{count, naive_count};

quickcheck! {
    fn check_counts_correctly(x: (Vec<u8>, u8)) -> bool {
        let (haystack, needle) = x;
        count(&haystack.clone(), needle) == naive_count(&haystack, needle)
    }
}

#[test]
fn check_large() {
    let haystack = vec![0u8; 10_000_000];
    assert_eq!(naive_count(&haystack, 0), count(&haystack, 0));
    assert_eq!(naive_count(&haystack, 1), count(&haystack, 1));
}

#[test]
fn check_some() {
    let haystack = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68];
    let needle = 68;
    assert_eq!(count(&haystack, needle), naive_count(&haystack, needle));
}
