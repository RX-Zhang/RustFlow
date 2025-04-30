use std::boxed::Box;

const NB_SUBBANDS: usize = 4;

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    // 其他字段省略...
}

fn aptx_update_codeword_history(channel: &mut Box<AptxChannel>) {
    // 实现省略...
}

fn aptx_generate_dither(channel: &mut Box<AptxChannel>) {
    aptx_update_codeword_history(channel);

    let m = 5184443i64.wrapping_mul((channel.codeword_history >> 7) as i64);
    let d = ((m.wrapping_mul(4)).wrapping_add(m >> 22)) as i32;

    for subband in 0..NB_SUBBANDS {
        channel.dither[subband] = (d as u32).wrapping_shl((23 - 5 * subband as u32) % 32) as i32;
    }

    channel.dither_parity = (d >> 25) & 1;
}
