
use std::num::Wrapping;

fn sum_pairwise_products(n: i32) -> i32 {
    let mut sum = Wrapping(0i64);
    for i in 1..=n {
        for j in i..=n {
            sum += Wrapping(i as i64) * Wrapping(j as i64);
        }
    }
    sum.0 as i32
}
