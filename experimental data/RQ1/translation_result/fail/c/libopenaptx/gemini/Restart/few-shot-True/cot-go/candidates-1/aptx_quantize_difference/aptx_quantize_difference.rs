use std::boxed::Box;

fn clip_intp2(a: i32, p: u32) -> i32 {
    if (a as u32 + (1 << p)) & !((2 << p) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding as i64) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i32
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}

fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: usize) -> i32 {
    let mut idx = 0;
    let mut i = nb_intervals >> 1;
    while i > 0 {
        if (factor as i64) * (intervals[idx + i] as i64) <= ((value as i64) << 24) {
            idx += i;
        }
        i >>= 1;
    }
    idx as i32
}

fn aptx_quantize_difference(quantize: &mut Box<AptxQuantize>, sample_difference: i32, dither: i32, quantization_factor: i32, tables: &AptxTables) {
    let intervals = &tables.quantize_intervals;
    let quantized_sample = aptx_bin_search(sample_difference.abs() >> 4, quantization_factor, intervals, tables.tables_size);

    let d = rshift32_clip24(((dither as i64 * dither as i64) >> 32) as i32, 7) - (1 << 23);
    let d = rshift64((d as i64) * (tables.quantize_dither_factors[quantized_sample as usize] as i64), 23) as i32;

    let mean = (intervals[quantized_sample as usize + 1] + intervals[quantized_sample as usize]) / 2;
    let interval = (intervals[quantized_sample as usize + 1] - intervals[quantized_sample as usize]) * (!(sample_difference < 0) as i32 | 1);

    let dithered_sample = rshift64_clip24((dither as i64) * (interval as i64) + ((clip_intp2(mean + d, 23) as i64) << 32), 32);
    let error = ((sample_difference.abs() as i64) << 20) - (dithered_sample as i64) * (quantization_factor as i64);
    quantize.error = rshift64(error, 23) as i32;
    quantize.error = quantize.error.abs();

    let mut parity_change = quantized_sample;
    if error < 0 {
        parity_change = parity_change.wrapping_sub(1);
    } else {
        parity_change = parity_change.wrapping_sub(1);
    }

    let inv = if sample_difference < 0 { -1 } else { 0 }; // Corrected line
    quantize.quantized_sample = quantized_sample ^ inv;
    quantize.quantized_sample_parity_change = parity_change ^ inv;
}


struct AptxTables {
    quantize_intervals: Box<[i32]>,
    invert_quantize_dither_factors: Box<[i32]>,
    quantize_dither_factors: Box<[i32]>,
    quantize_factor_select_offset: Box<[i16]>,
    tables_size: usize,
    factor_max: i32,
    prediction_order: i32,
}

struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}
