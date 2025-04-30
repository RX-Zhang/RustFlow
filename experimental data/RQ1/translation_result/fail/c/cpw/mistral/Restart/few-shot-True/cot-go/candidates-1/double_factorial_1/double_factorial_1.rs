

fn double_factorial_1(n: u32) -> u32 {
let mut res = 1;
for i in (1..=n).step_by(2) {
let mut tmp = res as u64;
tmp *= i as u64;
if tmp > u32::MAX as u64 {
return 0;
}
res = tmp as u32;
}
res
}

