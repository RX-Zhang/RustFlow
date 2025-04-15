
use std::num::Wrapping;

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_opl_compute_phase_step(block_freq: u32, multiple: u32, lfo_raw_pm: i32) -> u32 {
    let mut fnum = Wrapping(opl_emu_bitfield(block_freq, 0, 10) << 2);
    fnum += Wrapping((lfo_raw_pm as u32).wrapping_mul(opl_emu_bitfield(block_freq, 7, 3)) >> 1);
    fnum &= Wrapping(0xfff);
    let block = opl_emu_bitfield(block_freq, 10, 3);
    let phase_step = (fnum.0 << block) >> 2;
    (phase_step.wrapping_mul(multiple)) >> 1
}
