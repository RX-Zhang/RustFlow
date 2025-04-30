

fn number_digits_product_two_numbers(a: i32, b: i32) -> i32 {
    let mut count = 0;
    let p = (a.wrapping_mul(b)).abs();

    if p == 0 {
        return 1;
    }

    let mut temp = p;
    while temp > 0 {
        count += 1;
        temp = temp.wrapping_div(10);
    }

    count
}
