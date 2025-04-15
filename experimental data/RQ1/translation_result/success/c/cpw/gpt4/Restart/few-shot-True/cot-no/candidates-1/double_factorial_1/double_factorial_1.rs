
pub fn double_factorial_1(n: u32) -> u32 {
    let mut res: u32 = 1;
    let mut i: i32 = n as i32;

    while i >= 0 {
        if i == 0 || i == 1 {
            return res;
        }
        res = res.wrapping_mul(i as u32);
        i = i.wrapping_sub(2);
    }
    
    res
}
