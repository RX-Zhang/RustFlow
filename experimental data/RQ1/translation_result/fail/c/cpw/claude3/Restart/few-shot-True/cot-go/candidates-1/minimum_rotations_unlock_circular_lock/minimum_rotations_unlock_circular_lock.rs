
use std::cmp;

fn minimum_rotations_unlock_circular_lock(mut input: i32, mut unlock_code: i32) -> i32 {
    let mut rotation = 0;
    while input != 0 || unlock_code != 0 {
        let input_digit = input % 10;
        let code_digit = unlock_code % 10;
        rotation += cmp::min(
            (input_digit - code_digit).abs(),
            10 - (input_digit - code_digit).abs()
        );
        input /= 10;
        unlock_code /= 10;
    }
    rotation
}
