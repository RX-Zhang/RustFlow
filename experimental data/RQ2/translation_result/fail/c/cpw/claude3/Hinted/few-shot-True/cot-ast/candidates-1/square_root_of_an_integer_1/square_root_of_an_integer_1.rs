
pub fn square_root_of_an_integer_1(x: i32) -> i32 {
    if x == 0 || x == 1 {
        return x;
    }
    
    let mut start = 1;
    let mut end = x;
    let mut ans = 0;
    
    while start <= end {
        let mid = start.wrapping_add(end).wrapping_div(2);
        let mid_squared = mid.wrapping_mul(mid);
        
        if mid_squared == x {
            return mid;
        }
        
        if mid_squared < x {
            start = mid.wrapping_add(1);
            ans = mid;
        } else {
            end = mid.wrapping_sub(1);
        }
    }
    
    ans
}
