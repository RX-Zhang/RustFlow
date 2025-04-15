
#[inline]
fn clip(a: i32, amin: i32, amax: i32) -> i32 {
    if a < amin {
        return amin;
    } else if a > amax {
        return amax;
    } else {
        return a;
    }
}
