
fn count_even_length_binary_sequences_with_same_sum_of_first_and_second_half_bits_1(n: i32) -> i32 {
    let mut nCr: i64 = 1;
    let mut res: i64 = 1;
    for r in 1..=n {
        nCr = (nCr * (n as i64 + 1 - r as i64)) / r as i64;
        res += nCr * nCr;
    }
    res as i32
}
