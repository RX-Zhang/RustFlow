
use std::cmp::min;

fn minimum_rotations_unlock_circular_lock(input: i32, unlock_code: i32) -> i32 {
    let mut input = input;
    let mut unlock_code = unlock_code;
    let mut rotation = 0;

    while input != 0 || unlock_code != 0 {
        let input_digit = input % 10;
        let code_digit = unlock_code % 10;
        rotation += min(
            (input_digit - code_digit).abs() as i32,
            (10 - (input_digit - code_digit)).abs() as i32,
        );
        input /= 10;
        unlock_code /= 10;
    }

    rotation
}
