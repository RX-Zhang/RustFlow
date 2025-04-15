use std::num::Wrapping;

#[derive(Default)]
struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

impl Default for AptxPrediction {
    fn default() -> Self {
        AptxPrediction {
            prev_sign: [0; 2],
            s_weight: [0; 2],
            d_weight: [0 24],
            pos: 0,
            reconstructed_differences: [0; 48],
            previous_reconstructed_sample: 0,
            predicted_difference: 0,
            predicted_sample: 0,
        }
    }
}

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) -> Box<i32> {
    let rd1 = &mut prediction.reconstructed_differences[..order as usize];
    let rd2 = &mut prediction.reconstructed_differences[order as usize..];
    let p = prediction.pos as usize;

    rd1[p] = rd2[p];
    prediction.pos = Wrapping(prediction.pos).0.wrapping_add(1).wrapping_rem(order) as i32;
    let new_p = prediction.pos as usize;
    rd2[new_p] = reconstructed_difference;
    Box::new(rd2[new_p])
}
