
fn number_digits_product_two_numbers(a: i32, b: i32) -> i32 {
    let mut count = 0;
    let p = (a * b).abs();
    if p == 0 {
        return 1;
    }
    while p > 0 {
        count += 1;
        let p = p / 10;
    }
    count
}
