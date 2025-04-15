
fn reverse_bits(x: u32) -> u32 {
// Reverse the bits of x using bitwise operations
let mut y = 0;
let mut i = 32;
while i > 0 {
i -= 1;
if (x & (1 << i)) != 0 {
y |= 1 << (31 - i);
}
}
y
}
