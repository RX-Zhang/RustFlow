use std::mem::MaybeUninit;

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

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) -> Box<i32> {
    let rd1 = prediction.reconstructed_differences.as_mut_ptr();
    let rd2 = unsafe { rd1.add(order as usize) };
    let mut p = prediction.pos;

    unsafe {
        *rd1.add(p as usize) = *rd2.add(p as usize);
    }

    p = (p.wrapping_add(1) % order).wrapping_abs();
    prediction.pos = p;

    unsafe {
        *rd2.add(p as usize) = reconstructed_difference;
        Box::new(*rd2.add(p as usize))
    }
}
