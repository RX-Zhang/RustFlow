use std::num::Wrapping;

fn update_prediction(prediction: &mut Prediction, order: usize) {
    prediction.pos = (Wrapping(prediction.pos) + Wrapping(1)).0 % order;
}

struct Prediction {
    pos: usize,
}
