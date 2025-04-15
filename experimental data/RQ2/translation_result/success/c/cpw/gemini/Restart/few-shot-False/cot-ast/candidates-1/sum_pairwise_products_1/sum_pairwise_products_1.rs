
fn sum_pairwise_products_1(n: i32) -> i32 {
    let mut multi_terms = (n * (n + 1)) / 2;
    let mut sum = multi_terms;
    for i in 2..=n {
        multi_terms -= i - 1;
        sum += multi_terms * i;
    }
    sum
}
