
fn sum_pairwise_products(n: i32) -> i64 {
    let mut sum = Box::new(0i64);
    for i in 1..=n {
        for j in i..=n {
            *sum = sum.wrapping_add((i as i64) * (j as i64));
        }
    }
    *sum
}
