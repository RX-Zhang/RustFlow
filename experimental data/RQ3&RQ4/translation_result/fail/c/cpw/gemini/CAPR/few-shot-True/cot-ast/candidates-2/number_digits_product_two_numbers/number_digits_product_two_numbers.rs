
fn number_digits_product_two_numbers(a: i32, b: i32) -> i32 {
    let p = (a * b).abs();
    if p == 0 {
        return 1;
    }
    let mut count = 0;
    let mut p = p;
    while p > 0 {
        count += 1;
        p /= 10;
    }
    count
}
