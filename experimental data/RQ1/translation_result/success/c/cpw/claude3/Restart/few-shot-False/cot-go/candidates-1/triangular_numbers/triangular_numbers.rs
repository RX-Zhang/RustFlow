
fn triangular_numbers(num: i32) -> i32 {
    if num < 0 {
        return 0;
    }
    
    let mut sum: i32 = 0;
    let mut n: i32 = 1;
    
    while sum <= num {
        sum = sum.wrapping_add(n);
        if sum == num {
            return 1;
        }
        n = n.wrapping_add(1);
    }
    
    0
}
