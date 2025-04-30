fn number_digits_product_two_numbers(a: i32, b: i32) -> i32 {
    let mut count = 0;
    let mut p = (a.wrapping_mul(b)).wrapping_abs();
    if p == 0 {
        return 1;
    }
    while p > 0 {
        count += 1;
        p /= 10;
    }
    count
}