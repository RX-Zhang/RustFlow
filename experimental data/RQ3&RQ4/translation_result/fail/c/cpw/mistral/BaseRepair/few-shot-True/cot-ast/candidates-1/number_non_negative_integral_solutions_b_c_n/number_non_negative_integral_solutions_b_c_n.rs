
use std::i32;
use std::ops::Add;

fn number_non_negative_integral_solutions_b_c_n(n: i32) -> i32 {
let mut result = 0;
for i in 0..=n {
for j in 0..=n - i {
let k_end = n - i - j;
if (i.saturating_add(j).saturating_add(k_end as i32)) == n {
result += 1;
}
}
}
result
}
