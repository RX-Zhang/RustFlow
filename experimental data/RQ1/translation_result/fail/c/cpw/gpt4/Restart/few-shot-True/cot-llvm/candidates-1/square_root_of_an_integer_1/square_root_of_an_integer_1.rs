
fn square_root_of_an_integer_1(x: i32) -> i32 {
    if x == 0 || x == 1 {
        return x;
    }
    
    let mut start = 1;
    let mut end = x;
    let mut ans = 0;
    
    while start <= end {
        let mid = (start + end) / 2;
        
        if mid * mid == x {
            return mid;
        } else if mid * mid < x {
            start = mid.wrapping_add(1);
            ans = mid;
        } else {
            end = mid.wrapping_sub(1);
        }
    }
    
    ans
}
