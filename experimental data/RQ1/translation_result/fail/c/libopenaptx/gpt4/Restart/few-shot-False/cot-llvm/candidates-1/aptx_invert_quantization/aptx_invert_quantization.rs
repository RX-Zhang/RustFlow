fn aptx_invert_quantization(
    invert_quantize: &mut AptxInvertQuantize,
    quantized_sample: i32,
    dither: i32,
    tables: &AptxTables,
) {
    let mut idx = (quantized_sample ^ ((quantized_sample < 0) as i32).wrapping_neg()).wrapping_add(1);
    let mut qr = tables.quantize_intervals[idx as usize] / 2;
    if quantized_sample < 0 {
        qr = -qr;
    }

    qr = rshift64_clip24(
        ((qr as i64).wrapping_mul(1i64 << 32))
            .wrapping_add((dither as i64).wrapping_mul(tables.invert_quantize_dither_factors[idx as usize] as i64)),
        32,
    );
    invert_quantize.reconstructed_difference =
        ((invert_quantize.quantization_factor as i64).wrapping_mul(qr as i64) >> 19) as i32;

    let mut factor_select = 32620 * invert_quantize.factor_select;
    factor_select = rshift32(
        factor_select.wrapping_add((tables.quantize_factor_select_offset[idx as usize] as i32) << 15),
        15,
    );
    invert_quantize.factor_select = clip(factor_select, 0, tables.factor_max);

    idx = (invert_quantize.factor_select & 0xFF) >> 3;
    let shift = (tables.factor_max - invert_quantize.factor_select) >> 8;
    invert_quantize.quantization_factor =
        (QUANTIZATION_FACTORS[idx as usize] as i32) << 11 >> shift;
}
