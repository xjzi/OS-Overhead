#![no_std]

use blake2::{Blake2b, Digest};

pub fn work() {
    for _ in 0..10_000_000 {
        let mut hasher = Blake2b::new();
        hasher.update(b"Hashing is a good way to take up CPU time.");
        let _res = hasher.finalize();
    }
}