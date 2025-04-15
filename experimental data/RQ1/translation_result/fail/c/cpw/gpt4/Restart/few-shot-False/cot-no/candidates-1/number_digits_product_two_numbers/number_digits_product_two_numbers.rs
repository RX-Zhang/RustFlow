
use std::i32;

fn number_digits_product_two_numbers(a: i32, b: i32) -> usize {
    let mut count = 0;
    let product = a.wrapping_mul(b);

    // In Rust, instead of abs from stdlib.h, which might overflow, using i64
    let mut p = i64::from(product).abs();

    if p == 0 {
        return 1; // Return 1 if the product is 0.
    }

    while p > 0 {
        count += 1;
        p /= 10; // Rust automatically handles type conversion.
    }

    count
}

