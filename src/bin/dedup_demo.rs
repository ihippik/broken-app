use broken_app::{algo};
use std::hint::black_box;

fn main() {
    let data: Vec<u64> = (0..200_000).collect();

    let result = algo::fast_dedup(&data);
    black_box(result);
}