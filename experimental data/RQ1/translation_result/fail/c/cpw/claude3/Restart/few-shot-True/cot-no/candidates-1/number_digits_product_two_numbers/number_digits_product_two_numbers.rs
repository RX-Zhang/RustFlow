

fn number_digits_product_two_numbers(a: i32, b: i32) -> i32 {
    let mut count = 0;
    let p = a.wrapping_mul(b).abs() as u64;
    
    if p == 0 {
        return 1;
    }
    
    let mut p_mut = p;
    while p_mut > 0 {
        count += 1;
        p_mut /= 10;
    }
    
    count
}
