
fn check_number_is_perfect_square_using_additionsubtraction(n: i32) -> i32 {
    let mut sum = 0;
    let mut i = 1;
    
    while sum < n {
        sum = sum.wrapping_add(i);
        if sum == n {
            return 1;
        }
        i = i.wrapping_add(2);
    }
    
    0
}
