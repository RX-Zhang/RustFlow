fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1 << p)) & !((2 << p) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}
