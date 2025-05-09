fn count_even_length_binary_sequences_with_same_sum_of_first_and_second_half_bits_1(n: i32) -> i32 {
    let mut ncr: i32 = 1;
    let mut res: i32 = 1;
    for r in 1..=n {
        ncr = ncr.wrapping_mul((n + 1 - r)) / r;
        res = res.wrapping_add(ncr.wrapping_mul(ncr));
    }
    res
}