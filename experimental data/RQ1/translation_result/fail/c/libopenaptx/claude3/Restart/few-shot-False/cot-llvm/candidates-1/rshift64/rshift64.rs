#[inline]
pub fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64.wrapping_shl(shift.wrapping_sub(1));
    let mask = (1i64.wrapping_shl(shift.wrapping_add(1))).wrapping_sub(1);
    let tmp = value.wrapping_add(rounding);
    let tmp1 = tmp.wrapping_shr(shift as u32);
    let tmp2 = value & mask;
    let tmp3 = (tmp2 == rounding) as i64;
    tmp1.wrapping_sub(tmp3)
}
