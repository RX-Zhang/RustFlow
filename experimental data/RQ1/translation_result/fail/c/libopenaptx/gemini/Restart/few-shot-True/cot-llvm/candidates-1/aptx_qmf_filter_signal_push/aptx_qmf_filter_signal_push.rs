fn aptx_qmf_filter_signal_push(signal: &mut Box<[i32; 32]>, sample: i32) {
    let pos = (signal[1] as usize) % 16;
    signal[pos] = sample;
    signal[pos + 16] = sample;
    signal[1] = (pos + 1) as i32;
}

