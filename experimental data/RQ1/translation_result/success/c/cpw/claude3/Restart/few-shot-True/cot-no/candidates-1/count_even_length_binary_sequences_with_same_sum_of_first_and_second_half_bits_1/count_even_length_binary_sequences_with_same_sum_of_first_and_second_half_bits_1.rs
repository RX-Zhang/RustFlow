
fn count_even_length_binary_sequences_with_same_sum_of_first_and_second_half_bits_1(n: i32) -> i32 {
    let mut n_cr: i32 = 1;
    let mut res: i32 = 1;
    for r in 1..=n {
        n_cr = n_cr.wrapping_mul((n + 1 - r) as i32).wrapping_div(r);
        res = res.wrapping_add(n_cr.wrapping_mul(n_cr));
    }
    res
}
