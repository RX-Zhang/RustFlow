fn aptx_reset(ctx: &mut AptxContext) {
    let hd = ctx.hd;

    // Reset context using safe Rust methods
    *ctx = AptxContext {
        decode_sync_packets: 0,
        decode_dropped: 0,
        channels: [
            Box::new(AptxChannel {
                codeword_history: 0,
                dither_parity: 0,
                dither: [0; NB_SUBBANDS],
                qmf: Box::new(AptxQMFAnalysis {
                    outer_filter_signal: [AptxFilterSignal {
                        buffer: [0; 2 * FILTER_TAPS],
                        pos: 0,
                    }; NB_FILTERS],
                    inner_filter_signal: [[AptxFilterSignal {
                        buffer: [0; 2 * FILTER_TAPS],
                        pos: 0,
                    }; NB_FILTERS]; NB_FILTERS],
                }),
                quantize: [AptxQuantize {
                    quantized_sample: 0,
                    quantized_sample_parity_change: 0,
                    error: 0,
                }; NB_SUBBANDS],
                invert_quantize: [AptxInvertQuantize {
                    quantization_factor: 0,
                    factor_select: 0,
                    reconstructed_difference: 0,
                }; NB_SUBBANDS],
                prediction: [AptxPrediction {
                    prev_sign: [1, 1],
                    s_weight: [0; 2],
                    d_weight: [0; 24],
                    pos: 0,
                    reconstructed_differences: [0 48],
                    previous_reconstructed_sample: 0,
                    predicted_difference: 0,
                    predicted_sample: 0,
                }; NB_SUBBANDS],
            }); NB_CHANNELS],
        ],
        hd,
        sync_idx: 0,
        encode_remaining: ((LATENCY_SAMPLES + 3) / 4) as u8,
        decode_skip_leading: ((LATENCY_SAMPLES + 3) / 4) as u8,
        decode_sync_buffer_len: 0,
        decode_sync_buffer: [0; 6],
    };
}
