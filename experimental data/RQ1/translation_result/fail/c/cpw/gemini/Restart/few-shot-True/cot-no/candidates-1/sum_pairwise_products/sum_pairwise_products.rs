
fn sum_pairwise_products(n: i32) -> i32 {
    let mut sum: i64 = 0;
    for i in 1..=n {
        for j in i..=n {
            sum = sum.wrapping_add((i as i64).wrapping_mul(j as i64));
        }
    }
    sum as i32
}
