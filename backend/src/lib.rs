#![feature(try_blocks)]
#![feature(let_chains)]

use std::sync::atomic::{AtomicU32, Ordering};

pub mod classes;

pub fn increment(message: &str) -> u32 {
    static N: AtomicU32 = AtomicU32::new(0);

    println!("got a message on the server: {message}");
    N.fetch_add(1, Ordering::Relaxed)
}
