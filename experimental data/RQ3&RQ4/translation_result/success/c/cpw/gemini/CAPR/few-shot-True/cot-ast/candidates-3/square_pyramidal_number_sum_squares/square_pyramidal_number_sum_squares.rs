
fn square_pyramidal_number_sum_squares(s: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut n: i32 = 1;

    while sum < s {
        // Check if adding n^2 would cause overflow before performing the addition
        if let Some(new_sum) = sum.checked_add(n * n) {
            sum = new_sum;
        } else {
            return -1; // Overflow would occur, so return -1
        }

        if sum == s {
            return n;
        }
        n = n.wrapping_add(1);
    }

    -1
}
