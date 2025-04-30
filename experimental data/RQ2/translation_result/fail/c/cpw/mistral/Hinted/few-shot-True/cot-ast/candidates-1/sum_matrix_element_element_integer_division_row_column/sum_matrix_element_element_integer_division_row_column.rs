

use std::u32;

fn sum_matrix_element_element_integer_division_row_column(n: u32) -> u32 {
let mut ans: u32 = 0;
for i in 1..=n {
for j in 1..=n {
ans = ans.saturating_add(u32::wrapping_div(i, j));
}
}
ans
}

