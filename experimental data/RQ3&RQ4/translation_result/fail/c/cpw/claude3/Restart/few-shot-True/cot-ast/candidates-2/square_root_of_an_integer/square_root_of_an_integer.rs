
fn square_root_of_an_integer(x: i32) -> i32 {
    if x == 0 || x == 1 {
        return x;
    }
    
    let mut i: i32 = 1;
    let mut result: i32 = 1;
    
    while result <= x {
        i = i.wrapping_add(1);
        result = i.wrapping_mul(i);
    }
    
    i.wrapping_sub(1)
}
