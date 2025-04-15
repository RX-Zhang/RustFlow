
fn sum_pairwise_products(n: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..=n {
        for j in i..=n {
            sum = sum.wrapping_add(i * j);
        }
    }
    sum
}
