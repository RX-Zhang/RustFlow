
fn next_power_of_2(n: u32) -> u32 {
    let mut count: i32 = 0; // Specify the type as i32
    let mut val = n;
    if val != 0 && (val & (val.wrapping_sub(1))) == 0 {
        return val;
    }
    while val != 0 {
        val = val.wrapping_shr(1);
        count = count.wrapping_add(1);
    }
    1 << count
}
