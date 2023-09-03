//! fast finder of numbers with a specified amount of trailing zeros in their SHA-256 hash
use digital::NumToString;
use sha2::{Digest, Sha256};
use std::{
    sync::mpsc::{SendError, Sender},
    thread,
};

/// iterate numbers starting from `initial_value`
/// and if their SHA-256 hash ends with `trailing_zeros` trailing zeros
/// send them to `sender` using `num_threads` threads;
/// the order of the numbers is not guaranteed, but they are guaranteed not to repeat
pub fn hash_finder(
    sender: Sender<(u64, String)>,
    initial_value: u64,
    trailing_zeros: u8,
    num_threads: usize,
) {
    assert!(
        trailing_zeros < 64,
        "number of trailing zeros must be less than 64"
    );

    run_threads(
        |i| {
            let sender = sender.clone();
            move || {
                for (x, hash) in Sha256TrailingZerosIterator::new(
                    initial_value + i as u64,
                    num_threads as _,
                    trailing_zeros,
                ) {
                    if let Err(SendError { .. }) = sender.send((x, hash)) {
                        return;
                    }
                }
            }
        },
        num_threads,
    );
}

/// convert u64 to its SHA-256 hash
fn sha256_u64(x: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(x.to_heapless_string(false, false));
    let hash = hasher.finalize();
    format!("{hash:x}")
}

/// yields numbers with `trailing_zeros` trailing zeros in their
/// SHA-256 hash, and the hash itself (single-threaded)
pub struct Sha256TrailingZerosIterator {
    current: Option<u64>,
    step: u64,
    trailing_zeros: u8,
}

impl Sha256TrailingZerosIterator {
    fn new(initial_value: u64, step: u64, trailing_zeros: u8) -> Self {
        Self {
            current: Some(initial_value),
            step,
            trailing_zeros,
        }
    }
}

impl Iterator for Sha256TrailingZerosIterator {
    type Item = (u64, String);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current = self.current?;
            self.current = current.checked_add(self.step);
            let hash = sha256_u64(current);
            if hash.as_bytes()[hash.len() - self.trailing_zeros as usize..]
                .iter()
                .all(|&x| x == b'0')
            {
                return Some((current, hash));
            }
        }
    }
}

/// run `num_threads` threads that execute functions that `f` yields;
/// `f` takes the index of the current thread
fn run_threads<F: Fn(usize) -> G, G: FnOnce() + Send + 'static>(f: F, num_threads: usize) {
    let mut handles = Vec::with_capacity(num_threads);
    for i in 0..num_threads {
        handles.push(thread::spawn(f(i)));
    }
    for h in handles {
        h.join().expect("failed to join handle");
    }
}

#[cfg(test)]
mod tests {
    use crate::{hash_finder, run_threads, sha256_u64, Sha256TrailingZerosIterator};
    use core::sync::atomic::{AtomicU8, Ordering};
    use std::{
        sync::{mpsc::channel, Arc},
        thread,
    };

    #[test]
    fn test_sha256_u64() {
        assert_eq!(
            sha256_u64(0),
            "5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9"
        );
        assert_eq!(
            sha256_u64(456),
            "b3a8e0e1f9ab1bfe3a36f231f676f78bb30a519d2b21e6c530c0eee8ebb4a5d0"
        );
        assert_eq!(
            sha256_u64(12478198),
            "76c313563dd436161a882556639fcfa824211b3c5c0fff69bb62cbc92792e216"
        );
    }

    #[test]
    fn test_trailing_zeros_iterator() {
        let mut it = Sha256TrailingZerosIterator::new(0, 2, 4);
        assert_eq!(
            it.next(),
            Some((
                31214,
                "16b024b09ebcb9d66f6a9968858d7e01081e51a14a4922edf3c8e3c2009c0000".into()
            ))
        );
        assert_eq!(
            it.next(),
            Some((
                112370,
                "4ff6b83cd5d3afa354d1ae9cf8923e9cf6caa199cc3b74ee3c53b47da1c20000".into()
            ))
        );
        assert!(it.next().is_some());
    }

    #[test]
    fn test_run_threads() {
        let counter = Arc::new(AtomicU8::from(0));
        let n = 6;
        run_threads(
            |i| {
                let counter = counter.clone();
                move || {
                    counter.fetch_add(i as _, Ordering::Relaxed);
                }
            },
            n,
        );
        assert_eq!(counter.load(Ordering::SeqCst), (0..n as u8).sum::<u8>());
    }

    #[test]
    fn test_hash_finder() {
        let (sender, receiver) = channel();
        thread::spawn(|| hash_finder(sender, 1, 4, 6));
        for (n, hash) in receiver.into_iter().take(5) {
            assert_eq!(hash, sha256_u64(n));
            assert!(hash.ends_with("0000"));
        }
    }
}
