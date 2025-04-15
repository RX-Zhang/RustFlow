
fn number_digits_product_two_numbers(a: i32, b: i32) -> i32 {
    let p = match (a as i64).wrapping_mul(b as i64) {
        p if p > i32::MAX as i64 || p < i32::MIN as i64 => 0,
        p => (p.abs() as i32)
    };

    if p == 0 {
        return if a == 0 || b == 0 { 1 } else { 0 };
    }
    
    let mut count = 0;
    let mut current = p;
    while current > 0 {
        count += 1;
        current /= 10;
    }
    
    count
}
