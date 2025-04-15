pub fn opl_emu_clamp(value: i32, minval: i32, maxval: i32) -> i32 {
    if value < minval {
        return minval;
    }
    if value > maxval {
        return maxval;
    }
    value
}
