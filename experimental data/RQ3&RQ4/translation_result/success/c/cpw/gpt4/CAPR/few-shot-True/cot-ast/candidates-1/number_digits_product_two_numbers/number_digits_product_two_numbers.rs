
use std::i32;

fn number_digits_product_two_numbers(a: i32, b: i32) -> i32 {
    let mut count = 0;
    let p = (a as i64).wrapping_mul(b as i64).abs(); // Use i64 to handle potential overflow
    if p == 0 {
        return 1;
    }
    let mut p = Box::new(p); // Using Box pointers as per constraints
    while *p > 0 {
        count += 1;
        *p = p.wrapping_div(10);
    }
    count
}
