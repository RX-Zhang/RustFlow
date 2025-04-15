
fn dynamic_programming_set_36_cut_a_rope_to_maximize_product_1(n: i32) -> i32 {
    if n == 2 || n == 3 {
        return n - 1;
    }
    
    let mut res: i32 = 1;
    let mut n_mutable = n;
    
    while n_mutable > 4 {
        n_mutable -= 3;
        res = res.wrapping_mul(3);
    }
    
    n_mutable.wrapping_mul(res)
}
