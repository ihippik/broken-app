pub mod algo;
pub mod concurrency;

/// Returns the sum of all even values in the given slice.
pub fn sum_even(values: &[i64]) -> i64 {
    let mut acc = 0;
    for idx in 0..values.len() {
        let v = values[idx];

        if v % 2 == 0 {
            acc += v;
        }
    }
    acc
}

/// Counts the number of non-zero bytes in the input slice.
///
/// This implementation is fully safe and does not perform any allocations
/// or raw pointer manipulation.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|&&b| b != 0).count()
}

/// Normalizes whitespace and converts the string to lowercase.
pub fn normalize(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Returns the arithmetic mean of all positive values in the given slice.
///
/// If the slice contains no positive values, returns `0.0`.
pub fn average_positive(values: &[i64]) -> f64 {
    let mut sum = 0i64;
    let mut count = 0usize;

    for &v in values {
        if v > 0 {
            sum += v;
            count += 1;
        }
    }

    if count == 0 {
        return 0.0;
    }

    sum as f64 / count as f64
}

/// Fixed version of a use-after-free example.
pub unsafe fn use_after_free_fixed() -> i32 {
    let b = Box::new(42_i32);

    let a = *b;
    let c = *b;
    
    a + c
}
