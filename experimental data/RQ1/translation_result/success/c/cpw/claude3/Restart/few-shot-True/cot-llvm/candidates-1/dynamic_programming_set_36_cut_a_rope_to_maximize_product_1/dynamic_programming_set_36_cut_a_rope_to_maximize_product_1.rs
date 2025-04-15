
fn dynamic_programming_set_36_cut_a_rope_to_maximize_product_1(n: i32) -> i32 {
    if n == 2 || n == 3 {
        return n - 1;
    }
    
    let mut n = n;
    let mut res: i32 = 1;
    
    while n > 4 {
        n -= 3;
        res = res.wrapping_mul(3);
    }
    
    n.wrapping_mul(res)
}
