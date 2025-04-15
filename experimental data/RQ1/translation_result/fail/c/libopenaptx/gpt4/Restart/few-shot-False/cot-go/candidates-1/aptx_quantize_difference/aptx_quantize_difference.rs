fn aptx_quantize_difference(
    quantize: &mut AptxQuantize,
    sample_difference: i32,
    dither: i32,
    quantization_factor: i32,
    tables: &AptxTables,
) {
    let intervals = &tables.quantize_intervals;
    let mut sample_difference_abs = sample_difference.abs();

    if sample_difference_abs > ((1 << 23) - 1) {
        sample_difference_abs = (1 << 23) - 1;
    }

    let mut quantized_sample = aptx_bin_search(sample_difference_abs >> 4, quantization_factor, intervals, tables.tables_size);

    let mut d = rshift32_clip24(((((dither as i64).wrapping_mul(dither as i64)) >> 32) as i32).wrapping_add(1 << 23), 7);
    d = rshift64((d as i64).wrapping_mul(tables.quantize_dither_factors[quantized_sample as usize] as i64), 23) as i32;

    let intervals = &tables.quantize_intervals[quantized_sample as usize..];
    let mean = (intervals[1] + intervals[0]) / 2;
    let interval = (intervals[1] - intervals[0]) * if sample_difference < 0 { -1 } else { 1 };

    let dithered_sample = rshift64_clip24(
        (dither as i64).wrapping_mul(interval as i64) + ((clip_intp2(mean + d, 23) as i64) << 32),
        32,
    );

    let error = ((sample_difference_abs as i64) << 20) - (dithered_sample as i64).wrapping_mul(quantization_factor as i64);
    quantize.error = rshift64(error, 23) as i32;
    if quantize.error < 0 {
        quantize.error = -quantize.error;
    }

    let mut parity_change = quantized_sample;
    if error < 0 {
        quantized_sample -= 1;
    } else {
        parity_change -= 1;
    }

    let inv = if sample_difference < 0 { -1 } else { 0 };
    quantize.quantized_sample = quantized_sample ^ inv;
    quantize.quantized_sample_parity_change = parity_change ^ inv;
}
