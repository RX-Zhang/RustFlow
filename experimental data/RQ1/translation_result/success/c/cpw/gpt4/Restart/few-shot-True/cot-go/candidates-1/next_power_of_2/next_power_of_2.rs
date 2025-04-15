
fn next_power_of_2(n: u32) -> u32 {
    let mut count: u32 = 0;
    let mut num = n;
    
    if num != 0 && (num & (num.wrapping_sub(1))) == 0 {
        return num;
    }
    
    while num != 0 {
        num = num.wrapping_shr(1);
        count = count.wrapping_add(1);
    }
    
    1_u32.wrapping_shl(count)
}
