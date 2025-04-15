
use std::i32;

fn sum_matrix_element_element_integer_division_row1(n: i32) -> i32 {
let mut ans: i32 = 0;
let mut temp: i32 = 0;
let mut num: i32 = 1;
let mut i: i32 = 1;

while i <= n && temp < n {
temp = i as i32 - 1;

while temp < n {
if (temp + i) <= n {
ans = ans.wrapping_add((i * num) as i32);
} else {
ans = ans.wrapping_add(((n - temp) % i32::MAX as i32) * num);
}

temp = (temp + i) as i32;
num = num.wrapping_add(1);
}

i = i.wrapping_add(1) as i32;
}

ans
}
