use std::num::Wrapping;

fn update_rd2(rd2: &mut [i32], prediction: &Prediction, reconstructed_difference: i32) {
    let pos = Wrapping(prediction.pos as usize).0;
    rd2[pos] = reconstructed_difference;
}

struct Prediction {
    pos: i32,
}
