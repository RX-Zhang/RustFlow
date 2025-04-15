
use std::num::Wrapping;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_CHANNELS: usize = 9;
const OPL_EMU_REGISTERS_OPERATORS: usize = 2;

fn opl_emu_registers_generate_waveform(waveform: &mut [i32; OPL_EMU_REGISTERS_WAVEFORMS]) {
    for i in 0..OPL_EMU_REGISTERS_WAVEFORMS {
        let mut val = 0i32;
        for j in 0..256 {
            let phase = Wrapping(j as i32);
            let mask = Wrapping(-(((phase.0 >> 3) & 1) as i32));
            let shift = ((phase.0 >> 1) & 3) as u32;
            let mut sine = Wrapping(((phase.0 & 0x1f) as i32) << 7);

            sine = Wrapping(sine.0.wrapping_sub(0x400));
            sine = Wrapping(sine.0.wrapping_abs());
            sine = Wrapping(0x400 - sine.0);
            sine = Wrapping(sine.0.wrapping_shl(1));

            val = match i {
                0 => sine.0,
                1 => if phase.0 & 0x80 != 0 { sine.0 } else { 0 },
                2 | 3 => sine.0,
                4 => sine.0.wrapping_shr(shift),
                5 => if phase.0 & 0x80 != 0 { sine.0.wrapping_shr(shift) } else { 0 },
                6 => (mask.0 & sine.0.wrapping_shr(shift)) | (!mask.0 & sine.0),
                7 => sine.0.wrapping_shr(3),
                _ => 0,
            };

            waveform[i] = val;
        }
    }
}
