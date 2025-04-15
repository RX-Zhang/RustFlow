
fn check_if_a_number_is_jumbled_or_not(num: i32) -> i32 {
    if num.wrapping_div(10) == 0 {
        return 1;
    }
    
    let mut n = num;
    while n != 0 {
        if n.wrapping_div(10) == 0 {
            return 1;
        }
        
        let digit1 = n.wrapping_rem(10);
        let digit2 = (n.wrapping_div(10)).wrapping_rem(10);
        
        if (digit2 - digit1).abs() > 1 {
            return 0;
        }
        
        n = n.wrapping_div(10);
    }
    
    1
}
