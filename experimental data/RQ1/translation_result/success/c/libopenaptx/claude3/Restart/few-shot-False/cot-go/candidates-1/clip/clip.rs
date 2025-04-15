use std::num::Wrapping;

fn clip(a: i32, amin: i32, amax: i32) -> i32 {
    if Wrapping(a) < Wrapping(amin) {
        amin
    } else if Wrapping(a) > Wrapping(amax) {
        amax
    } else {
        a
    }
}
