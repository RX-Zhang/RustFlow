
fn sum_k_th_group_odd_positive_numbers(k: i32) -> i32 {
    let mut cur = k.wrapping_mul(k.wrapping_sub(1)).wrapping_add(1);
    let mut sum: i32 = 0; // Specify the type for sum
    let mut k = k;

    while k > 0 {
        sum = sum.wrapping_add(cur);
        cur = cur.wrapping_add(2);
        k -= 1;
    }
    sum
}
