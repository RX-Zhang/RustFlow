fn opl_min(a: u32, b: u32) -> u32 {
    if a < b { a } else { b }
}

fn opl_emu_registers_effective_rate(rawrate: u32, ksr: u32) -> u32 {
    if rawrate == 0 {
        0
    } else {
        opl_min(rawrate.wrapping_add(ksr), 63)
    }
}
