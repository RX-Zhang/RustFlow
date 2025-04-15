
fn double_factorial_1(n: u32) -> u32 {
    let mut res = 1;
    let mut i = n;
    
    loop {
        if i == 0 || i == 1 {
            return res;
        } else {
            res = res.wrapping_mul(i);
        }
        
        if i < 2 {
            break;
        }
        i = i.wrapping_sub(2);
    }
    res
}
