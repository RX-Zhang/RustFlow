
/// Returns the result of shifting the bits of `number` to the by `amount` positions.
/// If `amount` is greater than or equal to the width of `number`, the result is `number`
/// with its bits shifted all the way to the left and zero-extended to the width of `number`.
pub fn shiftleft(number: i32, amount: u32) -> i32 {
number.wrapping_shl(amount)
}

/// Returns the result of shifting the bits of `number` to the right by `amount` positions.
/// If `amount` is greater than or equal to the width of `number`, the result is `number`
/// with its bits shifted all the way to the right and sign-extended to the width of `number`.
pub fn shift_right(number: i32, amount: u32) -> i32 {
number.wrapping_shr(amount) as i32
}
