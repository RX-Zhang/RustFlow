
use std::i32;

fn sum_pairwise_products_1(n: i32) -> i32 {
    let mut multi_terms = n.wrapping_mul(n.wrapping_add(1)).wrapping_div(2);
    let mut sum = multi_terms;

    for i in 2..=n {
        multi_terms = multi_terms.wrapping_sub(i.wrapping_sub(1));
        sum = sum.wrapping_add(multi_terms * i);
    }

    sum
}
