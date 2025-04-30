
fn sum_k_th_group_odd_positive_numbers(mut k: i32) -> i32 {
    let mut cur = (k * (k - 1)) + 1;
    let mut sum = 0;
    while k > 0 {
        sum += cur;
        cur += 2;
        k -= 1;
    }
    sum
}
