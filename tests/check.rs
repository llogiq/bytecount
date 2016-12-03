extern crate bytecount;
#[macro_use]
extern crate quickcheck;
extern crate rand;

use std::iter;
use bytecount::{count, naive_count};
use rand::Rng;

fn random_bytes(len: usize) -> Vec<u8> {
    rand::thread_rng().gen_iter::<u8>().take(len).collect::<Vec<_>>()
}

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
fn check_large_rand() {
    let haystack = random_bytes(100_000);
    for i in (0..255).chain(iter::once(255)) {
        assert_eq!(naive_count(&haystack, i), count(&haystack, i));
    }
}

#[test]
fn check_some() {
    let haystack = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68];
    let needle = 68;
    assert_eq!(count(&haystack, needle), naive_count(&haystack, needle));
}

#[test]
fn check_overflow() {
    let haystack = vec![0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let needle = 2;
    assert_eq!(count(&haystack[0..], needle),
               naive_count(&haystack[0..], needle));
}
