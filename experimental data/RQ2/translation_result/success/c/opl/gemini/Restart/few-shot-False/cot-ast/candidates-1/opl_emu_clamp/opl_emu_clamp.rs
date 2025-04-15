
fn opl_emu_clamp(value: i32, minval: i32, maxval: i32) -> i32 {
    if value < minval {
        minval
    } else if value > maxval {
        maxval
    } else {
        value
    }
}
