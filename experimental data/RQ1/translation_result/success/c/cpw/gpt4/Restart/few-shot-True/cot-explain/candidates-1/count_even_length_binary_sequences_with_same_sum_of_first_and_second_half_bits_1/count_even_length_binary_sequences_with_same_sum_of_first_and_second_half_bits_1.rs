
fn count_even_length_binary_sequences_with_same_sum_of_first_and_second_half_bits_1(n: i32) -> i32 {
    let mut ncr = 1_i32;
    let mut res = 1_i32;
    
    for r in 1..=n {
        ncr = ncr * (n + 1 - r) / r;
        res = res.wrapping_add(ncr * ncr);
    }
    
    res
}
