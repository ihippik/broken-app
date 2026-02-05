use broken_app::{algo, leak_buffer, normalize, sum_even};

#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]); // порядок и состав важны
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "hello world");
}

#[test]
fn normalize_keeps_single_spaces_between_words() {
    assert_eq!(normalize("Hello   world"), "hello world");
}

#[test]
fn normalize_treats_tabs_like_spaces() {
    assert_eq!(normalize("Hello\tworld"), "hello world");
}

#[test]
fn normalize_collapses_mixed_whitespace_to_single_space() {
    assert_eq!(normalize("Hello \t  \t world"), "hello world");
}

#[test]
fn normalize_trims_outer_whitespace_but_preserves_word_separation() {
    assert_eq!(normalize("   Hello   world   "), "hello world");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    assert!((broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

#[test]
fn use_after_free_fixed_works() {
    let result = unsafe { broken_app::use_after_free_fixed() };
    assert_eq!(result, 84);
}
