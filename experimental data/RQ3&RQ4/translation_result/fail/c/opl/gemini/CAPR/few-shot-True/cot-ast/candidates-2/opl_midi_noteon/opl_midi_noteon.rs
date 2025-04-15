```rust
use std::cmp::min;
use std::ops::{BitAnd, BitOr, Not, Shl, Shr};

const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_ALL_CHANNELS: u32 = (1u32.wrapping_shl(OPL_EMU_REGISTERS_CHANNELS as u32)) - 1;
const OPL_EMU_REGISTERS_RHYTHM_CHANNEL: u32 = 0xff;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_REG_MODE: u16 = 0x04;
const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OP2_2NDVOICE_PRIORITY_PENALTY: i32 = 0xFF;

#[derive(Copy, Clone)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
    States = 6,
}

#[derive(Copy, Clone)]
enum OplEmuKeyonType {
    Normal = 0,
    Rhythm = 1,
    Csm = 2,
}

#[derive(Copy, Clone)]
enum Op2Flags {
    FixedPitch = 1,
    Unused = 2,
    DoubleVoice = 4,
}

struct OplT;

const OP2_OFFSETS: [u16; 18] = [
    0x03, 0x04, 0x05, 0x0b, 0x0c, 0x0d, 0x13, 0x14, 0x15, 0x103, 0x104, 0x105, 0x10b, 0x10c, 0x10d,
    0x113, 0x114, 0x115,
];

const FREQ_TABLE: [u16; 128] = [
    345, 365, 387, 410, 435, 460, 488, 517, 547, 580, 615, 651, 690, 731, 774, 820, 869, 921, 975, 517,
    547, 580, 615, 651, 690, 731, 774, 820, 869, 921, 975, 517, 547, 580, 615, 651, 690, 731, 774, 820,
    869, 921, 975, 517, 547, 580, 615, 651, 690, 731, 774, 820, 869, 921, 975, 517, 547, 580, 615, 651,
    690, 731, 774, 820, 869, 921, 975, 517, 547, 580, 615, 651, 690, 731, 774, 820, 869, 921, 975, 517,
    547, 580, 615, 651, 690, 731, 774, 820, 869, 921, 975, 517, 547, 580, 615, 651, 690, 731, 774, 820,
    869, 921, 975, 517, 547, 580, 615, 651, 690, 731, 774, 820, 869, 921, 975, 517,
];

const PITCH_TABLE: [u16; 256] = [
    29193, 29219, 29246, 29272, 29299, 29325, 29351, 29378, 29405, 29431, 29458, 29484, 29511, 29538,
    29564, 29591, 29618, 29644, 29671, 29698, 29725, 29752, 29778, 29805, 29832, 29859, 29886, 29913,
    29940, 29967, 29994, 30021, 30048, 30076, 30103, 30130, 30157, 30184, 30212, 30239, 30266, 30293,
    30321,