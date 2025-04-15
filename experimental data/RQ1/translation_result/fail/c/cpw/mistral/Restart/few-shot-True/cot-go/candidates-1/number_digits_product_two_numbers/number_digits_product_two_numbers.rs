
fn number_digits_product_two_numbers(a: i32, b: i32) -> i32 {
    let product = match a.checked_mul(b) {
        Some(p) => p.abs(),
        None => return 0  // Handle multiplication overflow
    };
    
    if product == 0 {
        return 1;
    }
    
    let mut count = 0;
    let mut p = product;
    while p > 0 {
        count += 1;
        p /= 10;
    }
    
    count
}
