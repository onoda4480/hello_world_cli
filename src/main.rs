use std::io::{stdout, Write};

fn main() {
    let out = stdout();
    let mut out = out.lock();
    for _ in 0..10_000_000 {
        write!(out, "").unwrap();
    }
}