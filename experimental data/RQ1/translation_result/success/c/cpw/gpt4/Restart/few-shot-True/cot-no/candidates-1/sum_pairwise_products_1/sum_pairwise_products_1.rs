
fn sum_pairwise_products_1(n: i32) -> i32 {
    let mut multi_terms = (n as i64) * ((n + 1) as i64) / 2;
    let mut sum = multi_terms;
    for i in 2..=n {
        multi_terms -= (i - 1) as i64;
        sum = sum.wrapping_add(multi_terms * (i as i64));
    }
    sum as i32
}
