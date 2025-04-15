
use std::num::Wrapping;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> (start as u32)) & ((1u32 << (length as u32)) - 1)
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
    *noise_lfsr |= opl_emu_bitfield(*noise_lfsr, 23, 1) ^ opl_emu_bitfield(*noise_lfsr, 9, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 8, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 1, 1);

    let am_counter = Wrapping(*lfo_am_counter);
    *lfo_am_counter = if am_counter.0 >= 210 * 64 - 1 {
        0
    } else {
        am_counter.0.wrapping_add(1)
    };

    let shift = 9u32.wrapping_sub(2u32.wrapping_mul(am_depth));
    *lfo_am = if am_counter.0 < 105 * 64 {
        (am_counter.0 >> shift) as u8
    } else {
        ((Wrapping(210 * 64 + 63) - am_counter).0 >> shift) as u8
    };

    *lfo_pm_counter = lfo_pm_counter.wrapping_add(1);

    let pm_scale: [i8; 8] = [8, 4, 0, -4, -8, -4, 0, 4];
    let pm_index = opl_emu_bitfield(*lfo_pm_counter as u32, 10, 3) as usize;
    let pm_value = pm_scale[pm_index] as i32;
    pm_value.wrapping_shr((pm_depth ^ 1) as u32)
}
