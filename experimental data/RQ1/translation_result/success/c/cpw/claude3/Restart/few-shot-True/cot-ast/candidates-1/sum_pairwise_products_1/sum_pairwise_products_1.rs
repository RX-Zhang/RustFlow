
fn sum_pairwise_products_1(n: i32) -> i32 {
    let mut multi_terms = n.wrapping_mul(n.wrapping_add(1)).wrapping_div(2) as i64;
    let mut sum = multi_terms;
    
    for i in 2..=n as i64 {
        multi_terms = multi_terms.wrapping_sub(i - 1);
        sum = sum.wrapping_add(multi_terms.wrapping_mul(i));
    }
    
    sum as i32
}
