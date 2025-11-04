#![allow(unused)]
use indicatif::ProgressBar;

fn main() {
    let pb = ProgressBar::new(100_000_000);
    for i in 0..100_000_000 {
        pb.inc(1);
    }
    pb.finish_with_message("done");
}