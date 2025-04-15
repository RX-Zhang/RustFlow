
use std::u32;
fn write_you_own_power_without_using_multiplication_and_division(a: i32, b: u32) -> i32 {
    if b == 0 {
        return 1;
    }

    let mut answer = a;
    let mut increment = a;

    for _ in 1..b {
        let temp_answer = answer;
        answer = 0;

        for _ in 1..a {
            answer = answer.wrapping_add(increment);
            increment = temp_answer;
        }
    }

    answer
}
