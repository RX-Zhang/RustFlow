fn aptx_qmf_tree_synthesis(
    qmf: &mut AptxQMFAnalysis,
    subband_samples: &[i32; NB_SUBBANDS],
    samples: &mut [i32; 4],
) {
    let mut intermediate_samples = [0; 4];

    for i in 0..2 {
        aptx_qmf_polyphase_synthesis(
            &mut qmf.inner_filter_signal[i],
            &APTX_QMF_INNER_COEFFS,
            22,
            subband_samples[2 * i],
            subband_samples[2 * i + 1],
            &mut intermediate_samples[2 * i..2 * i + 2],
        );
    }

    for i in 0..2 {
        aptx_qmf_polyphase_synthesis(
            &mut qmf.outer_filter_signal,
            &APTX_QMF_OUTER_COEFFS,
            21,
            intermediate_samples[0 + i],
            intermediate_samples[2 + i],
            &mut samples[2 * i..2 * i + 2],
        );
    }
}
