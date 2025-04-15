fn aptx_insert_sync(channels: &mut [AptxChannel; 2], sync_idx: &mut u8) {
    let mut min = &mut channels[1].quantize[0]; // Replaced NB_CHANNELS - 1 with 1
    let map = [1, 2, 0, 3];

    if aptx_check_parity(channels, sync_idx) != 0 {
        for c in channels.iter_mut().rev() {
            for &i in &map {
                if c.quantize[i].error < min.error {
                    min = &mut c.quantize[i];
                }
            }
        }
        min.quantized_sample = min.quantized_sample_parity_change;
    }
}
