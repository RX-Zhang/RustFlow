use std::num::Wrapping;

struct Opl {
    channelprog: [u8; 16],
    // Other fields would be defined here
}

fn opl_midi_changeprog(opl: &mut Opl, channel: i32, program: i32) {
    if channel == 9 {
        return;
    }
    opl.channelprog[channel as usize] = program as u8;
}
