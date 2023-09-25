//! # The Ideal Stocking Stuffer
//!
//! This solution relies on brute forcing combinations as quickly as possible using an internal
//! implementation of the [`MD5`] hashing algorithm.
//!
//! Each number's hash is independent of the others, so we speed things up by using threading
//! to search in parallel in blocks of 1000 numbers at a time.
//!
//! Using the [`format!`] macro to join the secret key to the number is quite slow. To go faster
//! we reuse the same `u8` buffer, incrementing digits one at a time.
//! The numbers from 1 to 999 are handled specially.
//!
//! Interestingly the total time to solve this problem is *extremely* sensitive to the secret key
//! provided as input. For example my key required ~10⁷ iterations to find the answer to part two.
//! However for unit testing, I was able to randomly find a value that takes only 455 iterations,
//! about 22,000 times faster!
//!
//! [`MD5`]: crate::util::md5
//! [`format!`]: std::format
use crate::util::md5::*;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;

pub struct Shared {
    prefix: String,
    done: Arc<AtomicBool>,
    counter: Arc<AtomicU32>,
    first: Arc<AtomicU32>,
    second: Arc<AtomicU32>,
}

pub fn parse(input: &str) -> Shared {
    let shared = Shared {
        prefix: input.trim().to_string(),
        done: Arc::new(AtomicBool::new(false)),
        counter: Arc::new(AtomicU32::new(1000)),
        first: Arc::new(AtomicU32::new(u32::MAX)),
        second: Arc::new(AtomicU32::new(u32::MAX)),
    };

    // Handle the first 999 numbers specially as the number of digits varies.
    for n in 1..1000 {
        let string = format!("{}{}", shared.prefix, n);
        check_hash(string.as_bytes(), n, &shared);
    }

    // Use as many cores as possible to parallelize the remaining search.
    thread::scope(|scope| {
        for _ in 0..thread::available_parallelism().unwrap().get() {
            scope.spawn(|| worker(&shared));
        }
    });

    shared
}

pub fn part1(input: &Shared) -> u32 {
    input.first.load(Ordering::Relaxed)
}

pub fn part2(input: &Shared) -> u32 {
    input.second.load(Ordering::Relaxed)
}

fn check_hash(buffer: &[u8], n: u32, shared: &Shared) {
    let (result, ..) = hash(buffer);

    if result & 0xffffff00 == 0 {
        shared.second.fetch_min(n, Ordering::Relaxed);
        shared.done.store(true, Ordering::Relaxed);
    } else if result & 0xfffff000 == 0 {
        shared.first.fetch_min(n, Ordering::Relaxed);
    }
}

fn worker(shared: &Shared) {
    while !shared.done.load(Ordering::Relaxed) {
        let offset = shared.counter.fetch_add(1000, Ordering::Relaxed);
        let string = format!("{}{}", shared.prefix, offset);
        let size = string.len() - 3;
        let mut buffer = string.as_bytes().to_vec();

        for n in 0..1000 {
            // Format macro is very slow, so update digits directly
            buffer[size] = b'0' + (n / 100) as u8;
            buffer[size + 1] = b'0' + ((n / 10) % 10) as u8;
            buffer[size + 2] = b'0' + (n % 10) as u8;
            check_hash(&buffer, offset + n, shared);
        }
    }
}
