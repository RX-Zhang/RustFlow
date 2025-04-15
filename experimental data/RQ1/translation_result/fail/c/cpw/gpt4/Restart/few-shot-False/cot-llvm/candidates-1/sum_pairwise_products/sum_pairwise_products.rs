
fn sum_pairwise_products(n: i32) -> i64 {
    let mut sum: i64 = 0;
    for i in 1..=n {
        for j in i..=n {
            sum = sum.wrapping_add((i * j) as i64);
        }
    }
    sum
}
