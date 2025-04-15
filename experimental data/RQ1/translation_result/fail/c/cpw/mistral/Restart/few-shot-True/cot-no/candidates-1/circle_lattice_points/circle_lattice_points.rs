
use std::i32;
use std::cmp::min;
use std::f64;

fn lattice_points(r: i32) -> i32 {
if r <= 0 {
0
} else {
let mut result = 4;
let mut x = 1;
while x < r {
let y_square = (r as f64 * r as f64 - x as f64 * x as f64) as i32;
let y = (f64::from(y_square).sqrt()).round() as i32;
if y * y == y_square {
result += 4;
}
x = x.wrapping_add(1);
}
result
}
}
