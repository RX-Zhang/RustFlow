use std::num::Wrapping;

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_opl_clock_noise_and_lfo(
    noise_lfsr: &mut u32,
    lfo_am_counter: &mut u16,
    lfo_pm_counter: &mut u16,
    lfo_am: &mut u8,
    am_depth: u32,
    pm_depth: u32,
) -> i32 {
    *noise_lfsr = noise_lfsr.wrapping_shl(1);
    *noise_lfsr |= opl_emu_bitfield(*noise_lfsr, 23, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 9, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 8, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 1, 1);

    let am_counter = Wrapping(*lfo_am_counter).0;
    *lfo_am_counter = lfo_am_counter.wrapping_add(1);
    if am_counter >= 210 * 64 - 1 {
        *lfo_am_counter = 0;
    }

    let shift = 9 - 2 * am_depth;
    *lfo_am = ((if am_counter < 105 * 64 {
        am_counter
    } else {
        210 * 64 + 63 - am_counter
    }) >> shift) as u8;

    let pm_counter = Wrapping(*lfo_pm_counter).0;
    *lfo_pm_counter = lfo_pm_counter.wrapping_add(1);

    let pm_scale: [i8; 8] = [8, 4, 0, -4, -8, -4, 0, 4];
    (pm_scale[opl_emu_bitfield(pm_counter.into(), 10, 3) as usize] >> (pm_depth.wrapping_rem(2) as i8)) as i32
}
