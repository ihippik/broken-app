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

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
pub fn leak_buffer(input: &[u8]) -> usize {
    let boxed = input.to_vec().into_boxed_slice();
    let len = input.len();
    let raw = Box::into_raw(boxed) as *mut u8;

    let mut count = 0;
    unsafe {
        for i in 0..len {
            if *raw.add(i) != 0_u8 {
                count += 1;
            }
        }
        // утечка: не вызываем Box::from_raw(raw);
    }
    count
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

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.
pub unsafe fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    let raw = Box::into_raw(b);
    let val = *raw;
    drop(Box::from_raw(raw));
    val + *raw
}
